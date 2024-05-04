use std::fmt::{self, Debug, Formatter};

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
	elem, Args, Cast, Construct, Content, NativeElement, Packed, Set, Smart, StyleChain,
	Unlabellable,
};
use crate::layout::{Em, Fragment, Length, Size};

/// Arranges text, spacing and inline-level elements into an inline element.
///
/// Although this function is primarily used in set rules to affect inline element
/// properties, it can also be used to explicitly render its argument onto a
/// inline element of its own.
///
/// # Example
/// ```example
/// #show inline: set block(spacing: 0.65em)
/// #set inline(
///   justify: true,
/// )
///
/// We proceed by contradiction.
/// Suppose that there exists a set
/// of positive integers $a$, $b$, and
/// $c$ that satisfies the equation
/// $a^n + b^n = c^n$ for some
/// integer value of $n > 2$.
///
/// Without loss of generality,
/// let $a$ be the smallest of the
/// three integers. Then, we ...
/// ```
#[elem(title = "Inline", Debug, Construct)]
pub struct InlineElem {
	/// The spacing between lines.
	#[resolve]
	#[ghost]
	#[default(Em::new(0.65).into())]
	pub leading: Length,

	/// Whether to justify text in its line.
	///
	/// Hyphenation will be enabled for justified paragraphs if the
	/// [text function's `hyphenate` property]($text.hyphenate) is set to
	/// `{auto}` and the current language is known.
	///
	/// Note that the current [alignment]($align.alignment) still has an effect
	/// on the placement of the last line except if it ends with a
	/// [justified line break]($linebreak.justify).
	#[ghost]
	#[default(false)]
	pub justify: bool,

	/// How to determine line breaks.
	///
	/// When this property is set to `{auto}`, its default value, optimized line
	/// breaks will be used for justified paragraphs. Enabling optimized line
	/// breaks for ragged paragraphs may also be worthwhile to improve the
	/// appearance of the text.
	///
	/// ```example
	/// #set page(width: 207pt)
	/// #set par(linebreaks: "simple")
	/// Some texts feature many longer
	/// words. Those are often exceedingly
	/// challenging to break in a visually
	/// pleasing way.
	///
	/// #set par(linebreaks: "optimized")
	/// Some texts feature many longer
	/// words. Those are often exceedingly
	/// challenging to break in a visually
	/// pleasing way.
	/// ```
	#[ghost]
	pub linebreaks: Smart<Linebreaks>,

	/// Indicates wheter an overflowing line should be shrunk.
	///
	/// This property is set to `false` on raw blocks, because shrinking a line
	/// could visually break the indentation.
	#[ghost]
	#[internal]
	#[default(true)]
	pub shrink: bool,

	/// The contents of the inline element.
	#[external]
	#[required]
	pub body: Content,

	/// The inline element's children.
	#[internal]
	#[variadic]
	pub children: Vec<Content>,
}

impl Construct for InlineElem {
	fn construct(engine: &mut Engine, args: &mut Args) -> SourceResult<Content> {
		// The inline constructor is special: It doesn't create an inline element
		// element. Instead, it just ensures that the passed content lives in a
		// separate inline and styles it.
		let styles = Self::set(engine, args)?;
		let body = args.expect::<Content>("body")?;
		Ok(body.styled_with_map(styles))
	}
}

impl Packed<InlineElem> {
	/// Layout the inline content into a collection of lines.
	#[typst_macros::time(name = "inline", span = self.span())]
	pub fn layout(
		&self,
		engine: &mut Engine,
		styles: StyleChain,
		consecutive: bool,
		region: Size,
		expand: bool,
	) -> SourceResult<Fragment> {
		crate::layout::layout_inline(
			self.children(),
			engine,
			styles,
			region,
			expand,
		)
	}
}

impl Debug for InlineElem {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Inline ")?;
		f.debug_list().entries(&self.children).finish()
	}
}

/// How to determine line breaks in an inline element.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum Linebreaks {
	/// Determine the line breaks in a simple first-fit style.
	Simple,
	/// Optimize the line breaks for the whole inline element.
	///
	/// Typst will try to produce more evenly filled lines of text by
	/// considering the whole inline element when calculating line breaks.
	Optimized,
}
