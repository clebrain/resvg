// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use strict_num::NonZeroPositiveF32;

use crate::{Fill, PaintOrder, Stroke, TextRendering, Visibility};

/// A font stretch property.
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Default for FontStretch {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

/// A font style property.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

impl Default for FontStyle {
    #[inline]
    fn default() -> FontStyle {
        Self::Normal
    }
}

/// Text font properties.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Font {
    /// A list of family names.
    ///
    /// Never empty. Uses `usvg_parser::Options::font_family` as fallback.
    pub families: Vec<String>,
    /// A font style.
    pub style: FontStyle,
    /// A font stretch.
    pub stretch: FontStretch,
    /// A font width.
    pub weight: u16,
}

/// A dominant baseline property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DominantBaseline {
    Auto,
    UseScript,
    NoChange,
    ResetSize,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Central,
    Middle,
    TextAfterEdge,
    TextBeforeEdge,
}

impl Default for DominantBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

/// An alignment baseline property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AlignmentBaseline {
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
}

impl Default for AlignmentBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

/// A baseline shift property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BaselineShift {
    Baseline,
    Subscript,
    Superscript,
    Number(f32),
}

impl Default for BaselineShift {
    #[inline]
    fn default() -> BaselineShift {
        BaselineShift::Baseline
    }
}

/// A length adjust property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LengthAdjust {
    Spacing,
    SpacingAndGlyphs,
}

impl Default for LengthAdjust {
    fn default() -> Self {
        Self::Spacing
    }
}

/// A text span decoration style.
///
/// In SVG, text decoration and text it's applied to can have different styles.
/// So you can have black text and green underline.
///
/// Also, in SVG you can specify text decoration stroking.
#[derive(Clone, Debug)]
pub struct TextDecorationStyle {
    /// A fill style.
    pub fill: Option<Fill>,
    /// A stroke style.
    pub stroke: Option<Stroke>,
}

/// A text span decoration.
#[derive(Clone, Debug)]
pub struct TextDecoration {
    /// An optional underline and its style.
    pub underline: Option<TextDecorationStyle>,
    /// An optional overline and its style.
    pub overline: Option<TextDecorationStyle>,
    /// An optional line-through and its style.
    pub line_through: Option<TextDecorationStyle>,
}

/// A text style span.
///
/// Spans do not overlap inside a text chunk.
#[derive(Clone, Debug)]
pub struct TextSpan {
    /// A span start in bytes.
    ///
    /// Offset is relative to the parent text chunk and not the parent text element.
    pub start: usize,
    /// A span end in bytes.
    ///
    /// Offset is relative to the parent text chunk and not the parent text element.
    pub end: usize,
    /// A fill style.
    pub fill: Option<Fill>,
    /// A stroke style.
    pub stroke: Option<Stroke>,
    /// A paint order style.
    pub paint_order: PaintOrder,
    /// A font.
    pub font: Font,
    /// A font size.
    pub font_size: NonZeroPositiveF32,
    /// Indicates that small caps should be used.
    ///
    /// Set by `font-variant="small-caps"`
    pub small_caps: bool,
    /// Indicates that a kerning should be applied.
    ///
    /// Supports both `kerning` and `font-kerning` properties.
    pub apply_kerning: bool,
    /// A span decorations.
    pub decoration: TextDecoration,
    /// A span dominant baseline.
    pub dominant_baseline: DominantBaseline,
    /// A span alignment baseline.
    pub alignment_baseline: AlignmentBaseline,
    /// A list of all baseline shift that should be applied to this span.
    ///
    /// Ordered from `text` element down to the actual `span` element.
    pub baseline_shift: Vec<BaselineShift>,
    /// A visibility property.
    pub visibility: Visibility,
    /// A letter spacing property.
    pub letter_spacing: f32,
    /// A word spacing property.
    pub word_spacing: f32,
    /// A text length property.
    pub text_length: Option<f32>,
    /// A length adjust property.
    pub length_adjust: LengthAdjust,
}

/// A text chunk anchor property.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

impl Default for TextAnchor {
    fn default() -> Self {
        Self::Start
    }
}

/// A path used by text-on-path.
#[derive(Clone, Debug)]
pub struct TextPath {
    /// A text offset in SVG coordinates.
    ///
    /// Percentage values already resolved.
    pub start_offset: f32,

    /// A path.
    pub path: Rc<tiny_skia_path::Path>,
}

/// A text chunk flow property.
#[derive(Clone, Debug)]
pub enum TextFlow {
    /// A linear layout.
    ///
    /// Includes left-to-right, right-to-left and top-to-bottom.
    Linear,
    /// A text-on-path layout.
    Path(Rc<TextPath>),
}

/// A text chunk.
///
/// Text alignment and BIDI reordering can only be done inside a text chunk.
#[derive(Clone, Debug)]
pub struct TextChunk {
    /// An absolute X axis offset.
    pub x: Option<f32>,
    /// An absolute Y axis offset.
    pub y: Option<f32>,
    /// A text anchor.
    pub anchor: TextAnchor,
    /// A list of text chunk style spans.
    pub spans: Vec<TextSpan>,
    /// A text chunk flow.
    pub text_flow: TextFlow,
    /// A text chunk actual text.
    pub text: String,
}

/// A text character position.
///
/// _Character_ is a Unicode codepoint.
#[derive(Clone, Copy, Debug)]
pub struct CharacterPosition {
    /// An absolute X axis position.
    pub x: Option<f32>,
    /// An absolute Y axis position.
    pub y: Option<f32>,
    /// A relative X axis offset.
    pub dx: Option<f32>,
    /// A relative Y axis offset.
    pub dy: Option<f32>,
}

/// A writing mode.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WritingMode {
    LeftToRight,
    TopToBottom,
}

/// A text element.
///
/// `text` element in SVG.
#[derive(Clone, Debug)]
pub struct Text {
    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Isn't automatically generated.
    /// Can be empty.
    pub id: String,

    /// Rendering mode.
    ///
    /// `text-rendering` in SVG.
    pub rendering_mode: TextRendering,

    /// A list of character positions.
    ///
    /// One position for each Unicode codepoint. Aka `char` in Rust.
    pub positions: Vec<CharacterPosition>,

    /// A list of rotation angles.
    ///
    /// One angle for each Unicode codepoint. Aka `char` in Rust.
    pub rotate: Vec<f32>,

    /// A writing mode.
    pub writing_mode: WritingMode,

    /// A list of text chunks.
    pub chunks: Vec<TextChunk>,
}
