use yew::{html::IntoPropValue, virtual_dom::AttrValue};

use self::{color::Color, image::Image, border::BorderWidth, border::BorderStyle, quantities::{Length, Duration}};

pub mod color;
pub mod border;
pub mod image;
pub mod url;
pub mod quantities;

#[derive(Clone)]
pub struct TODO;

impl IntoCss for TODO {
    fn into_css(self) -> String {
        todo!("This type is no yet implemented properly")
    }
}

// #[derive(Clone)]
// pub struct Shorthand<T>(T);

#[derive(Clone)]
pub enum CssField<T: IntoCss> {
    None,
    Some(T),
    Initial,
    Inherit
}

pub trait IntoCss {
    fn into_css(self) -> String;
}

impl<T: IntoCss> IntoCss for Option<T> {
    fn into_css(self) -> String {
        match self {
            Some(x) => x.into_css(),
            None => String::new()
        }
    }
}

impl<A: IntoCss, B: IntoCss, C: IntoCss> IntoCss for (A, B, C) {
    fn into_css(self) -> String {
        let (a, b, c) = self;
        format!("{} {} {}", a.into_css(), b.into_css(), c.into_css())
    }
}

impl<A: IntoCss, B: IntoCss, C: IntoCss, D: IntoCss> IntoCss for (A, B, C, D) {
    fn into_css(self) -> String {
        let (a, b, c, d) = self;
        format!("{} {} {} {}", a.into_css(), b.into_css(), c.into_css(), d.into_css())
    }
}

impl<T: IntoCss> CssField<T> {
    fn or(self, other: CssField<T>) -> CssField<T> {
        match self {
            Self::None => other,
            _ => self
        }
    }
}

impl<T: IntoCss> Default for CssField<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T: IntoCss> IntoCss for CssField<T> {
    fn into_css(self) -> String {
        match self {
            Self::None => "".to_string(),
            Self::Inherit => "inherit".to_string(),
            Self::Initial => "initial".to_string(),
            Self::Some(x) => x.into_css()
        }
    }
}

#[macro_export]
macro_rules! css_enum {
    ($name:ident; $($variant:tt $($display_name:literal)?)|*) => {
        #[derive(Clone, Copy)]
        pub enum $name {
            $($variant),*
        }

        impl IntoCss for $name {
            fn into_css(self) -> String {
                match self {
                    $($name::$variant => $crate::css_enum_item!($variant $($display_name)?).to_string()),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! css_enum_item {
    ($variant:tt) => {
        stringify!($variant).to_ascii_lowercase()
    };
    ($_:tt $display_name:literal) => {
        $display_name
    };
}


impl IntoCss for () {
    fn into_css(self) -> String {
        String::new()
    }
}

impl<T: IntoCss> From<Option<T>> for CssField<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(t) => CssField::Some(t),
            None => CssField::None
        }
    }
}

impl<A: IntoCss> From<(A,)> for CssField<A> {
    fn from((a,): (A,)) -> Self {
        CssField::Some(a)
    }
}

impl<A: IntoCss, B: IntoCss, C: IntoCss> From<(A, B)> for CssField<(CssField<A>, CssField<B>, CssField<C>)> {
    fn from((a, b): (A, B)) -> Self {
        CssField::Some((CssField::Some(a), CssField::Some(b), CssField::None))
    }
}

impl<A: IntoCss, B: IntoCss, C: IntoCss> From<(A, B, C)> for CssField<(CssField<A>, CssField<B>, CssField<C>)> {
    fn from((a, b, c): (A, B, C)) -> Self {
        CssField::Some((CssField::Some(a), CssField::Some(b), CssField::Some(c)))
    }
}

impl<A: IntoCss, B: IntoCss, C: IntoCss, D: IntoCss> From<(A, B, C)> for CssField<(CssField<A>, CssField<B>, CssField<C>, CssField<D>)> {
    fn from((a, b, c): (A, B, C)) -> Self {
        CssField::Some((CssField::Some(a), CssField::Some(b), CssField::Some(c), CssField::None))
    }
}

impl<A: IntoCss, B: IntoCss, C: IntoCss, D: IntoCss> From<(A,)> for CssField<(CssField<A>, CssField<B>, CssField<C>, CssField<D>)> {
    fn from((a,): (A,)) -> Self {
        CssField::Some((CssField::Some(a.into()), CssField::None, CssField::None, CssField::None))
    }
}

impl<A: IntoCss, B: IntoCss, C: IntoCss> std::fmt::Display for CssField<(Option<A>, Option<B>, Option<C>)> 
    where A: std::fmt::Display + Clone,
    B: std::fmt::Display + Clone,
    C: std::fmt::Display + Clone    
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssField::Some((a, b, c)) => {
                if let Some(x) = a {
                    write!(f, "{x}")?
                }
                if let Some(x) = b {
                    write!(f, " {x}")?
                }
                if let Some(x) = c {
                    write!(f, " {x}")?
                }
            },
            _ => ()
        }
        
        Ok(())
    }
}

macro_rules! implCSSBlockField {
    ($first:ty, $($type_name:ty),+) => {
        CssField<(CssField<$first>, $(CssField<$type_name>),+)>
    };
    ($type_name:ty) => {
        CssField<$type_name>
    };
}

macro_rules! implCSSBlock {
    ($($display:literal = $field_name:ident: $($type_name:ty),+)*) => {
        #[derive(Default, Clone)]
        pub struct CssBlock {
            $(
                pub $field_name: implCSSBlockField!($($type_name),+),
            )*
        }

        impl CssBlock {

            pub fn update(&mut self, other: &CssBlock) {
                $(
                    self.$field_name = other.$field_name.clone().or(self.$field_name.clone());
                )*
            }

            pub fn with(&self, other: &CssBlock) -> Self {
                let mut x = self.clone();
                x.update(other);
                x
            }
        }

        impl std::fmt::Display for CssBlock {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                $(match self.$field_name.clone() {
                    CssField::None => Ok(()),
                    x => write!(f, "{}: {};", $display, x.into_css()),
                }?;)*
                core::result::Result::Ok(())
            }
        }
    };
}


css_enum!(Position; Static | Relative | Absolute | Fixed);
css_enum!(Display;
    Inline | Block | ListItem "list-item" | InlineBlock "inline-block" | 
    Table | InlineTable "inline-table" | TableRowGroup "table-row-group" | 
    TableHeaderGroup "table-header-group" | TableFooterGroup "table-footer-group" | 
    TableRow "table-row" | TableColumnGroup "table-column-group" | TableColumn "table-column" |
    TableCell "table-cell" | TableCaption "table-caption" | None
);
css_enum!(AlignContent; 
    FlexStart "flex-start" | FlexEnd "flex-end" | Center | SpaceBetween "space-between" | 
    SpaceAround "space-around" | Stretch
);
css_enum!(AlignItems; FlexStart "flex-start" | FlexEnd "flex-end" | Center | Baseline | Stretch);
css_enum!(AlignSelf; Auto | FlexStart "flex-start" | FlexEnd "flex-end" | Center | Baseline | Stretch);
css_enum!(AlignmentBaseline; 
    Baseline | TextBottom "text-bottom" | Alphabetic | Ideographic | 
    Middle | Central | Mathematical | TextTop "text-top"
);
css_enum!(Float; Left | Right);

#[macro_export(local_inner_macros)]
macro_rules! css_line {
    // ($target:ident; border: $width:tt;) => {
    //     $target.border(Some($width.into()), None, None);
    // };
    // ($target:ident; border: $width:tt $style:tt;) => {
    //     $target.border(Some($width.into()), Some($style.into()), None);
    // };
    ($target:ident; $name:ident: $a:tt;) => {
        $target.$name = ($a.into(),).into();
    };
    ($target:ident; $name:ident: $a:tt $b:tt ;) => {
        $target.$name = ($a.into(), $b.into()).into();
    };
    ($target:ident; $name:ident: $a:tt $b:tt $c:tt ;) => {
        $target.$name = ($a.into(), $b.into(), $c.into()).into();
    };
    ($target:ident; $name:ident: $a:tt $b:tt $c:tt $d:tt ;) => {
        $target.$name = ($a.into(), $b.into(), $c.into(), $d.into()).into();
    };
}

#[macro_export]
macro_rules! css {
    ($($name:ident: $($value:expr) +;)*) => ({
        let mut x = $crate::style::CssBlock::default();
        $(
            $crate::css_line!(x; $name: $($value)+ ;);
        )*
        x
    });
}


impl IntoPropValue<Option<AttrValue>> for CssBlock {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.to_string().into())
    }
}

pub trait IntoStyle {
    fn into_style(self) -> CssBlock;
}

// struct SassItem {
//     query: String,
//     css: CssBlock,
//     children: Vec<Box<SassItem>>,
// }

// "position" = position: Position [Static | Relative | Absolute | Fixed | Inherit]
implCSSBlock! {

    "accent-color" = accent_color: Color
    "align-content" = align_content: AlignContent
    "align-items" = align_items: AlignItems
    "align-self" = align_self: AlignSelf
    "alignment-baseline" = alignment_baseline: AlignmentBaseline
    "all" = all: () // all only allows initial, inherit or unset so it has no inner type
    "animation" = animation: TODO //TODO: Type
    "animation-delay" = animation_delay: Duration
    "animation-direction" = animation_direction: TODO //TODO: Type
    "animation-duration" = animation_duration: Duration //TODO: Type
    "animation-fill-mode" = animation_fill_mode: TODO //TODO: Type
    "animation-iteration-count" = animation_iteration_count: TODO //TODO: Type
    "animation-name" = animation_name: TODO //TODO: Type
    "animation-play-state" = animation_play_state: TODO //TODO: Type
    "animation-timing-function" = animation_timing_function: TODO //TODO: Type
    "appearance" = appearance: TODO //TODO: Type
    "aspect-ratio" = aspect_ratio: TODO //TODO: Type
    "azimuth" = azimuth: TODO //TODO: Type
    "backface-visibility" = backface_visibility: TODO //TODO: Type
    "background" = background: TODO //TODO: Type
    "background-attachment" = background_attachment: TODO //TODO: Type
    "background-blend-mode" = background_blend_mode: TODO //TODO: Type
    "background-clip" = background_clip: TODO //TODO: Type
    "background-color" = background_color: Color
    "background-image" = background_image: Image
    "background-origin" = background_origin: TODO //TODO: Type
    "background-position" = background_position: TODO //TODO: Type
    "background-repeat" = background_repeat: TODO //TODO: Type
    "background-size" = background_size: TODO //TODO: Type
    "baseline-shift" = baseline_shift: TODO //TODO: Type
    "baseline-source" = baseline_source: TODO //TODO: Type
    "block-ellipsis" = block_ellipsis: TODO //TODO: Type
    "block-size" = block_size: TODO //TODO: Type
    "block-step" = block_step: TODO //TODO: Type
    "block-step-align" = block_step_align: TODO //TODO: Type
    "block-step-insert" = block_step_insert: TODO //TODO: Type
    "block-step-round" = block_step_round: TODO //TODO: Type
    "block-step-size" = block_step_size: TODO //TODO: Type
    "bookmark-label" = bookmark_label: TODO //TODO: Type
    "bookmark-level" = bookmark_level: TODO //TODO: Type
    "bookmark-state" = bookmark_state: TODO //TODO: Type    
    "border" = border: BorderWidth, BorderStyle, Color
    "border-block" = border_block: BorderWidth, BorderStyle, Color
    "border-block-color" = border_block_color: Color
    "border-block-end" = border_block_end: BorderWidth, BorderStyle, Color
    "border-block-end-color" = border_block_end_color: Color
    "border-block-end-style" = border_block_end_style: BorderStyle
    "border-block-end-width" = border_block_end_width: Length
    "border-block-start" = border_block_start: BorderWidth, BorderStyle, Color
    "border-block-start-color" = border_block_start_color: Color
    "border-block-start-style" = border_block_start_style: BorderStyle
    "border-block-start-width" = border_block_start_width: Length
    "border-block-style" = border_block_style: BorderStyle
    "border-block-width" = border_block_width: BorderWidth
    "border-bottom" = border_bottom: BorderWidth, BorderStyle, Color
    "border-bottom-color" = border_bottom_color: Color
    "border-bottom-left-radius" = border_bottom_left_radius: Length
    "border-bottom-right-radius" = border_bottom_right_radius: Length
    "border-bottom-style" = border_bottom_style: BorderStyle
    "border-bottom-width" = border_bottom_width: BorderWidth
    "border-boundary" = border_boundary: TODO //TODO: Type
    "border-collapse" = border_collapse: TODO //TODO: Type
    "border-color" = border_color: Color
    "border-end-end-radius" = border_end_end_radius: Length
    "border-end-start-radius" = border_end_start_radius: Length    
    "border-image" = border_image: Image
    "border-image-outset" = border_image_outset: Length
    "border-image-repeat" = border_image_repeat: TODO //TODO: Type
    "border-image-slice" = border_image_slice: TODO //TODO: Type
    "border-image-source" = border_image_source: TODO //TODO: Type
    "border-image-width" = border_image_width: BorderWidth
    "border-inline" = border_inline: BorderWidth, BorderStyle, Color
    "border-inline-color" = border_inline_color: Color
    "border-inline-end" = border_inline_end: BorderWidth, BorderStyle, Color
    "border-inline-end-color" = border_inline_end_color: Color
    "border-inline-end-style" = border_inline_end_style: BorderStyle
    "border-inline-end-width" = border_inline_end_width: BorderWidth
    "border-inline-start" = border_inline_start: BorderWidth, BorderStyle, Color
    "border-inline-start-color" = border_inline_start_color: Color
    "border-inline-start-style" = border_inline_start_style: BorderStyle
    "border-inline-start-width" = border_inline_start_width: Length
    "border-inline-style" = border_inline_style: BorderStyle
    "border-inline-width" = border_inline_width: BorderWidth    
    "border-left" = border_left: BorderWidth, BorderStyle, Color
    "border-left-color" = border_left_color: Color
    "border-left-style" = border_left_style: BorderStyle
    "border-left-width" = border_left_width: BorderWidth
    "border-radius" = border_radius: Length, Length, Length, Length
    "border-right" = border_right: BorderWidth, BorderStyle, Color
    "border-right-color" = border_right_color: Color
    "border-right-style" = border_right_style: BorderStyle
    "border-right-width" = border_right_width: BorderWidth
    "border-spacing" = border_spacing: TODO //TODO: Type
    "border-start-end-radius" = border_start_end_radius: Length
    "border-start-start-radius" = border_start_start_radius: Length
    "border-style" = border_style: BorderStyle
    "border-top" = border_top: BorderWidth, BorderStyle, Color
    "border-top-color" = border_top_color: Color
    "border-top-left-radius" = border_top_left_radius: Length
    "border-top-right-radius" = border_top_right_radius: Length
    "border-top-style" = border_top_style: BorderStyle
    "border-top-width" = border_top_width: BorderWidth
    "border-width" = border_width: BorderWidth
    "bottom" = bottom: Length
    "box-decoration-break" = box_decoration_break: TODO //TODO: Type
    "box-shadow" = box_shadow: TODO //TODO: Type
    "box-sizing" = box_sizing: TODO //TODO: Type
    "box-snap" = box_snap: TODO //TODO: Type
    "break-after" = break_after: TODO //TODO: Type
    "break-before" = break_before: TODO //TODO: Type
    "break-inside" = break_inside: TODO //TODO: Type
    "caption-side" = caption_side: TODO //TODO: Type
    "caret" = caret: TODO //TODO: Type
    "caret-color" = caret_color: Color
    "caret-shape" = caret_shape: TODO //TODO: Type
    "chains" = chains: TODO //TODO: Type
    "clear" = clear: TODO //TODO: Type
    "clip" = clip: TODO //TODO: Type
    "clip-path" = clip_path: TODO //TODO: Type
    "clip-rule" = clip_rule: TODO //TODO: Type
    "color" = color: Color
    "color-adjust" = color_adjust: TODO //TODO: Type
    "color-interpolation-filters" = color_interpolation_filters: TODO //TODO: Type
    "color-scheme" = color_scheme: TODO //TODO: Type
    "column-count" = column_count: TODO //TODO: Type
    "column-fill" = column_fill: TODO //TODO: Type
    "column-gap" = column_gap: TODO //TODO: Type
    "column-rule" = column_rule: TODO //TODO: Type
    "column-rule-color" = column_rule_color: Color
    "column-rule-style" = column_rule_style: TODO //TODO: Type
    "column-rule-width" = column_rule_width: Length
    "column-span" = column_span: TODO //TODO: Type
    "column-width" = column_width: Length
    "columns" = columns: TODO //TODO: Type
    "contain" = contain: TODO //TODO: Type
    "contain-intrinsic-block-size" = contain_intrinsic_block_size: TODO //TODO: Type
    "contain-intrinsic-height" = contain_intrinsic_height: Length
    "contain-intrinsic-inline-size" = contain_intrinsic_inline_size: TODO //TODO: Type
    "contain-intrinsic-size" = contain_intrinsic_size: TODO //TODO: Type
    "contain-intrinsic-width" = contain_intrinsic_width: Length
    "container" = container: TODO //TODO: Type
    "container-name" = container_name: TODO //TODO: Type
    "container-type" = container_type: TODO //TODO: Type
    "content" = content: TODO //TODO: Type
    "content-visibility" = content_visibility: TODO //TODO: Type
    "continue" = _continue: TODO //TODO: Type
    "counter-increment" = counter_increment: TODO //TODO: Type
    "counter-reset" = counter_reset: TODO //TODO: Type
    "counter-set" = counter_set: TODO //TODO: Type
    "cue" = cue: TODO //TODO: Type
    "cue-after" = cue_after: TODO //TODO: Type
    "cue-before" = cue_before: TODO //TODO: Type
    "cursor" = cursor: TODO //TODO: Type
    "direction" = direction: TODO //TODO: Type
    "display" = display: Display
    "dominant-baseline" = dominant_baseline: TODO //TODO: Type
    "elevation" = elevation: TODO //TODO: Type
    "empty-cells" = empty_cells: TODO //TODO: Type
    "fill" = fill: TODO //TODO: Type
    "fill-break" = fill_break: TODO //TODO: Type
    "fill-color" = fill_color: Color
    "fill-image" = fill_image: Image
    "fill-opacity" = fill_opacity: TODO //TODO: Type
    "fill-origin" = fill_origin: TODO //TODO: Type
    "fill-position" = fill_position: TODO //TODO: Type
    "fill-repeat" = fill_repeat: TODO //TODO: Type
    "fill-rule" = fill_rule: TODO //TODO: Type
    "fill-size" = fill_size: TODO //TODO: Type
    "filter" = filter: TODO //TODO: Type
    "flex" = flex: TODO //TODO: Type
    "flex-basis" = flex_basis: TODO //TODO: Type
    "flex-direction" = flex_direction: TODO //TODO: Type
    "flex-flow" = flex_flow: TODO //TODO: Type
    "flex-grow" = flex_grow: TODO //TODO: Type
    "flex-shrink" = flex_shrink: TODO //TODO: Type
    "flex-wrap" = flex_wrap: TODO //TODO: Type
    "float" = float: Float //TODO: Type
    "float-defer" = float_defer: TODO //TODO: Type
    "float-offset" = float_offset: TODO //TODO: Type
    "float-reference" = float_reference: TODO //TODO: Type
    "flood-color" = flood_color: Color
    "flood-opacity" = flood_opacity: TODO //TODO: Type
    "flow" = flow: TODO //TODO: Type
    "flow-from" = flow_from: TODO //TODO: Type
    "flow-into" = flow_into: TODO //TODO: Type
    "font" = font: TODO //TODO: Type
    "font-family" = font_family: TODO //TODO: Type
    "font-feature-settings" = font_feature_settings: TODO //TODO: Type
    "font-kerning" = font_kerning: TODO //TODO: Type
    "font-language-override" = font_language_override: TODO //TODO: Type
    "font-optical-sizing" = font_optical_sizing: TODO //TODO: Type
    "font-palette" = font_palette: TODO //TODO: Type
    "font-size" = font_size: TODO //TODO: Type
    "font-size-adjust" = font_size_adjust: TODO //TODO: Type
    "font-stretch" = font_stretch: TODO //TODO: Type
    "font-style" = font_style: TODO //TODO: Type
    "font-synthesis" = font_synthesis: TODO //TODO: Type
    "font-synthesis-small-caps" = font_synthesis_small_caps: TODO //TODO: Type
    "font-synthesis-style" = font_synthesis_style: TODO //TODO: Type
    "font-synthesis-weight" = font_synthesis_weight: TODO //TODO: Type
    "font-variant" = font_variant: TODO //TODO: Type
    "font-variant-alternates" = font_variant_alternates: TODO //TODO: Type
    "font-variant-caps" = font_variant_caps: TODO //TODO: Type
    "font-variant-east-asian" = font_variant_east_asian: TODO //TODO: Type
    "font-variant-emoji" = font_variant_emoji: TODO //TODO: Type
    "font-variant-ligatures" = font_variant_ligatures: TODO //TODO: Type
    "font-variant-numeric" = font_variant_numeric: TODO //TODO: Type
    "font-variant-position" = font_variant_position: TODO //TODO: Type
    "font-variation-settings" = font_variation_settings: TODO //TODO: Type
    "font-weight" = font_weight: TODO //TODO: Type
    "footnote-display" = footnote_display: TODO //TODO: Type
    "footnote-policy" = footnote_policy: TODO //TODO: Type
    "forced-color-adjust" = forced_color_adjust: TODO //TODO: Type
    "gap" = gap: TODO //TODO: Type
    "glyph-orientation-vertical" = glyph_orientation_vertical: TODO //TODO: Type
    "grid" = grid: TODO //TODO: Type
    "grid-area" = grid_area: TODO //TODO: Type
    "grid-auto-columns" = grid_auto_columns: TODO //TODO: Type
    "grid-auto-flow" = grid_auto_flow: TODO //TODO: Type
    "grid-auto-rows" = grid_auto_rows: TODO //TODO: Type
    "grid-column" = grid_column: TODO //TODO: Type
    "grid-column-end" = grid_column_end: TODO //TODO: Type
    "grid-column-start" = grid_column_start: TODO //TODO: Type
    "grid-row" = grid_row: TODO //TODO: Type
    "grid-row-end" = grid_row_end: TODO //TODO: Type
    "grid-row-start" = grid_row_start: TODO //TODO: Type
    "grid-template" = grid_template: TODO //TODO: Type
    "grid-template-areas" = grid_template_areas: TODO //TODO: Type
    "grid-template-columns" = grid_template_columns: TODO //TODO: Type
    "grid-template-rows" = grid_template_rows: TODO //TODO: Type
    "hanging-punctuation" = hanging_punctuation: TODO //TODO: Type
    "height" = height: Length
    "hyphenate-character" = hyphenate_character: TODO //TODO: Type
    "hyphenate-limit-chars" = hyphenate_limit_chars: TODO //TODO: Type
    "hyphenate-limit-last" = hyphenate_limit_last: TODO //TODO: Type
    "hyphenate-limit-lines" = hyphenate_limit_lines: TODO //TODO: Type
    "hyphenate-limit-zone" = hyphenate_limit_zone: TODO //TODO: Type
    "hyphens" = hyphens: TODO //TODO: Type
    "image-orientation" = image_orientation: TODO //TODO: Type
    "image-rendering" = image_rendering: TODO //TODO: Type
    "image-resolution" = image_resolution: TODO //TODO: Type
    "initial-letter" = initial_letter: TODO //TODO: Type
    "initial-letter-align" = initial_letter_align: TODO //TODO: Type
    "initial-letter-wrap" = initial_letter_wrap: TODO //TODO: Type
    "inline-size" = inline_size: TODO //TODO: Type
    "inline-sizing" = inline_sizing: TODO //TODO: Type
    "input-security" = input_security: TODO //TODO: Type
    "inset" = inset: TODO //TODO: Type
    "inset-block" = inset_block: TODO //TODO: Type
    "inset-block-end" = inset_block_end: TODO //TODO: Type
    "inset-block-start" = inset_block_start: TODO //TODO: Type
    "inset-inline" = inset_inline: TODO //TODO: Type
    "inset-inline-end" = inset_inline_end: TODO //TODO: Type
    "inset-inline-start" = inset_inline_start: TODO //TODO: Type
    "isolation" = isolation: TODO //TODO: Type
    "justify-content" = justify_content: TODO //TODO: Type
    "justify-items" = justify_items: TODO //TODO: Type
    "justify-self" = justify_self: TODO //TODO: Type
    "leading-trim" = leading_trim: TODO //TODO: Type
    "left" = left: Length
    "letter-spacing" = letter_spacing: TODO //TODO: Type
    "lighting-color" = lighting_color: Color
    "line-break" = line_break: TODO //TODO: Type
    "line-clamp" = line_clamp: TODO //TODO: Type
    "line-grid" = line_grid: TODO //TODO: Type
    "line-height" = line_height: Length
    "line-height-step" = line_height_step: TODO //TODO: Type
    "line-padding" = line_padding: TODO //TODO: Type
    "line-snap" = line_snap: TODO //TODO: Type
    "list-style" = list_style: TODO //TODO: Type
    "list-style-image" = list_style_image: Image
    "list-style-position" = list_style_position: TODO //TODO: Type
    "list-style-type" = list_style_type: TODO //TODO: Type
    "margin" = margin: Length
    "margin-block" = margin_block: Length
    "margin-block-end" = margin_block_end: Length
    "margin-block-start" = margin_block_start: Length
    "margin-bottom" = margin_bottom: Length
    "margin-break" = margin_break: Length
    "margin-inline" = margin_inline: Length
    "margin-inline-end" = margin_inline_end: Length
    "margin-inline-start" = margin_inline_start: Length
    "margin-left" = margin_left: Length
    "margin-right" = margin_right: Length
    "margin-top" = margin_top: Length
    "margin-trim" = margin_trim: Length
    "marker" = marker: TODO //TODO: Type
    "marker-end" = marker_end: TODO //TODO: Type
    "marker-knockout-left" = marker_knockout_left: Length
    "marker-knockout-right" = marker_knockout_right: Length
    "marker-mid" = marker_mid: TODO //TODO: Type
    "marker-pattern" = marker_pattern: TODO //TODO: Type
    "marker-segment" = marker_segment: TODO //TODO: Type
    "marker-side" = marker_side: TODO //TODO: Type
    "marker-start" = marker_start: TODO //TODO: Type
    "mask" = mask: TODO //TODO: Type
    "mask-border" = mask_border: TODO //TODO: Type
    "mask-border-mode" = mask_border_mode: TODO //TODO: Type
    "mask-border-outset" = mask_border_outset: TODO //TODO: Type
    "mask-border-repeat" = mask_border_repeat: TODO //TODO: Type
    "mask-border-slice" = mask_border_slice: TODO //TODO: Type
    "mask-border-source" = mask_border_source: TODO //TODO: Type
    "mask-border-width" = mask_border_width: Length
    "mask-clip" = mask_clip: TODO //TODO: Type
    "mask-composite" = mask_composite: TODO //TODO: Type
    "mask-image" = mask_image: Image
    "mask-mode" = mask_mode: TODO //TODO: Type
    "mask-origin" = mask_origin: TODO //TODO: Type
    "mask-position" = mask_position: TODO //TODO: Type
    "mask-repeat" = mask_repeat: TODO //TODO: Type
    "mask-size" = mask_size: TODO //TODO: Type
    "mask-type" = mask_type: TODO //TODO: Type
    "max-block-size" = max_block_size: TODO //TODO: Type
    "max-height" = max_height: Length
    "max-inline-size" = max_inline_size: TODO //TODO: Type
    "max-lines" = max_lines: TODO //TODO: Type
    "max-width" = max_width: Length
    "min-block-size" = min_block_size: TODO //TODO: Type
    "min-height" = min_height: Length
    "min-inline-size" = min_inline_size: TODO //TODO: Type
    "min-intrinsic-sizing" = min_intrinsic_sizing: TODO //TODO: Type
    "min-width" = min_width: Length
    "mix-blend-mode" = mix_blend_mode: TODO //TODO: Type
    "nav-down" = nav_down: TODO //TODO: Type
    "nav-left" = nav_left: Length
    "nav-right" = nav_right: Length
    "nav-up" = nav_up: TODO //TODO: Type
    "object-fit" = object_fit: TODO //TODO: Type
    "object-position" = object_position: TODO //TODO: Type
    "object-view-box" = object_view_box: TODO //TODO: Type
    "offset" = offset: TODO //TODO: Type
    "offset-anchor" = offset_anchor: TODO //TODO: Type
    "offset-distance" = offset_distance: TODO //TODO: Type
    "offset-path" = offset_path: TODO //TODO: Type
    "offset-position" = offset_position: TODO //TODO: Type
    "offset-rotate" = offset_rotate: TODO //TODO: Type
    "opacity" = opacity: TODO //TODO: Type
    "order" = order: TODO //TODO: Type
    "orphans" = orphans: TODO //TODO: Type
    "outline" = outline: TODO //TODO: Type
    "outline-color" = outline_color: Color
    "outline-offset" = outline_offset: TODO //TODO: Type
    "outline-style" = outline_style: TODO //TODO: Type
    "outline-width" = outline_width: Length
    "overflow" = overflow: TODO //TODO: Type
    "overflow-anchor" = overflow_anchor: TODO //TODO: Type
    "overflow-block" = overflow_block: TODO //TODO: Type
    "overflow-clip-margin" = overflow_clip_margin: Length
    "overflow-inline" = overflow_inline: TODO //TODO: Type
    "overflow-wrap" = overflow_wrap: TODO //TODO: Type
    "overflow-x" = overflow_x: TODO //TODO: Type
    "overflow-y" = overflow_y: TODO //TODO: Type
    "overscroll-behavior" = overscroll_behavior: TODO //TODO: Type
    "overscroll-behavior-block" = overscroll_behavior_block: TODO //TODO: Type
    "overscroll-behavior-inline" = overscroll_behavior_inline: TODO //TODO: Type
    "overscroll-behavior-x" = overscroll_behavior_x: TODO //TODO: Type
    "overscroll-behavior-y" = overscroll_behavior_y: TODO //TODO: Type
    "padding" = padding: Length
    "padding-block" = padding_block: Length
    "padding-block-end" = padding_block_end: Length
    "padding-block-start" = padding_block_start: Length
    "padding-bottom" = padding_bottom: Length
    "padding-inline" = padding_inline: Length
    "padding-inline-end" = padding_inline_end: Length
    "padding-inline-start" = padding_inline_start: Length
    "padding-left" = padding_left: Length
    "padding-right" = padding_right: Length
    "padding-top" = padding_top: Length
    "page" = page: TODO //TODO: Type
    "page-break-after" = page_break_after: TODO //TODO: Type
    "page-break-before" = page_break_before: TODO //TODO: Type
    "page-break-inside" = page_break_inside: TODO //TODO: Type
    "pause" = pause: TODO //TODO: Type
    "pause-after" = pause_after: TODO //TODO: Type
    "pause-before" = pause_before: TODO //TODO: Type
    "perspective" = perspective: TODO //TODO: Type
    "perspective-origin" = perspective_origin: TODO //TODO: Type
    "pitch" = pitch: TODO //TODO: Type
    "pitch-range" = pitch_range: TODO //TODO: Type
    "place-content" = place_content: TODO //TODO: Type
    "place-items" = place_items: TODO //TODO: Type
    "place-self" = place_self: TODO //TODO: Type
    "play-during" = play_during: TODO //TODO: Type
    "pointer-events" = pointer_events: TODO //TODO: Type
    "position" = position: TODO //TODO: Type
    "print-color-adjust" = print_color_adjust: TODO //TODO: Type
    "property" = property: TODO //TODO: Type
    "property-name" = property_name: TODO //TODO: Type
    "quotes" = quotes: TODO //TODO: Type
    "region-fragment" = region_fragment: TODO //TODO: Type
    "resize" = resize: TODO //TODO: Type
    "rest" = rest: TODO //TODO: Type
    "rest-after" = rest_after: TODO //TODO: Type
    "rest-before" = rest_before: TODO //TODO: Type
    "richness" = richness: TODO //TODO: Type
    "right" = right: Length
    "rotate" = rotate: TODO //TODO: Type
    "row-gap" = row_gap: TODO //TODO: Type
    "ruby-align" = ruby_align: TODO //TODO: Type
    "ruby-merge" = ruby_merge: TODO //TODO: Type
    "ruby-overhang" = ruby_overhang: TODO //TODO: Type
    "ruby-position" = ruby_position: TODO //TODO: Type
    "running" = running: TODO //TODO: Type
    "scale" = scale: TODO //TODO: Type
    "scroll-behavior" = scroll_behavior: TODO //TODO: Type
    "scroll-margin" = scroll_margin: Length
    "scroll-margin-block" = scroll_margin_block: TODO //TODO: Type
    "scroll-margin-block-end" = scroll_margin_block_end: TODO //TODO: Type
    "scroll-margin-block-start" = scroll_margin_block_start: TODO //TODO: Type
    "scroll-margin-bottom" = scroll_margin_bottom: Length
    "scroll-margin-inline" = scroll_margin_inline: TODO //TODO: Type
    "scroll-margin-inline-end" = scroll_margin_inline_end: TODO //TODO: Type
    "scroll-margin-inline-start" = scroll_margin_inline_start: TODO //TODO: Type
    "scroll-margin-left" = scroll_margin_left: Length
    "scroll-margin-right" = scroll_margin_right: Length
    "scroll-margin-top" = scroll_margin_top: Length
    "scroll-padding" = scroll_padding: TODO //TODO: Type
    "scroll-padding-block" = scroll_padding_block: TODO //TODO: Type
    "scroll-padding-block-end" = scroll_padding_block_end: TODO //TODO: Type
    "scroll-padding-block-start" = scroll_padding_block_start: TODO //TODO: Type
    "scroll-padding-bottom" = scroll_padding_bottom: Length
    "scroll-padding-inline" = scroll_padding_inline: TODO //TODO: Type
    "scroll-padding-inline-end" = scroll_padding_inline_end: TODO //TODO: Type
    "scroll-padding-inline-start" = scroll_padding_inline_start: TODO //TODO: Type
    "scroll-padding-left" = scroll_padding_left: Length
    "scroll-padding-right" = scroll_padding_right: Length
    "scroll-padding-top" = scroll_padding_top: Length
    "scroll-snap-align" = scroll_snap_align: TODO //TODO: Type
    "scroll-snap-stop" = scroll_snap_stop: TODO //TODO: Type
    "scroll-snap-type" = scroll_snap_type: TODO //TODO: Type
    "scrollbar-color" = scrollbar_color: Color
    "scrollbar-gutter" = scrollbar_gutter: TODO //TODO: Type
    "scrollbar-width" = scrollbar_width: Length
    "shape-image-threshold" = shape_image_threshold: TODO //TODO: Type
    "shape-inside" = shape_inside: TODO //TODO: Type
    "shape-margin" = shape_margin: Length
    "shape-outside" = shape_outside: TODO //TODO: Type
    "spatial-navigation-action" = spatial_navigation_action: TODO //TODO: Type
    "spatial-navigation-contain" = spatial_navigation_contain: TODO //TODO: Type
    "spatial-navigation-function" = spatial_navigation_function: TODO //TODO: Type
    "speak" = speak: TODO //TODO: Type
    "speak-as" = speak_as: TODO //TODO: Type
    "speak-header" = speak_header: TODO //TODO: Type
    "speak-numeral" = speak_numeral: TODO //TODO: Type
    "speak-punctuation" = speak_punctuation: TODO //TODO: Type
    "speech-rate" = speech_rate: TODO //TODO: Type
    "stress" = stress: TODO //TODO: Type
    "string-set" = string_set: TODO //TODO: Type
    "stroke" = stroke: TODO //TODO: Type
    "stroke-align" = stroke_align: TODO //TODO: Type
    "stroke-alignment" = stroke_alignment: TODO //TODO: Type
    "stroke-break" = stroke_break: TODO //TODO: Type
    "stroke-color" = stroke_color: Color
    "stroke-dash-corner" = stroke_dash_corner: TODO //TODO: Type
    "stroke-dash-justify" = stroke_dash_justify: TODO //TODO: Type
    "stroke-dashadjust" = stroke_dashadjust: TODO //TODO: Type
    "stroke-dasharray" = stroke_dasharray: TODO //TODO: Type
    "stroke-dashcorner" = stroke_dashcorner: TODO //TODO: Type
    "stroke-dashoffset" = stroke_dashoffset: TODO //TODO: Type
    "stroke-image" = stroke_image: Image
    "stroke-linecap" = stroke_linecap: TODO //TODO: Type
    "stroke-linejoin" = stroke_linejoin: TODO //TODO: Type
    "stroke-miterlimit" = stroke_miterlimit: TODO //TODO: Type
    "stroke-opacity" = stroke_opacity: TODO //TODO: Type
    "stroke-origin" = stroke_origin: TODO //TODO: Type
    "stroke-position" = stroke_position: TODO //TODO: Type
    "stroke-repeat" = stroke_repeat: TODO //TODO: Type
    "stroke-size" = stroke_size: TODO //TODO: Type
    "stroke-width" = stroke_width: Length
    "tab-size" = tab_size: TODO //TODO: Type
    "table-layout" = table_layout: TODO //TODO: Type
    "text-align" = text_align: TODO //TODO: Type
    "text-align-all" = text_align_all: TODO //TODO: Type
    "text-align-last" = text_align_last: TODO //TODO: Type
    "text-combine-upright" = text_combine_upright: TODO //TODO: Type
    "text-decoration" = text_decoration: TODO //TODO: Type
    "text-decoration-color" = text_decoration_color: Color
    "text-decoration-line" = text_decoration_line: TODO //TODO: Type
    "text-decoration-skip" = text_decoration_skip: TODO //TODO: Type
    "text-decoration-skip-box" = text_decoration_skip_box: TODO //TODO: Type
    "text-decoration-skip-ink" = text_decoration_skip_ink: TODO //TODO: Type
    "text-decoration-skip-inset" = text_decoration_skip_inset: TODO //TODO: Type
    "text-decoration-skip-self" = text_decoration_skip_self: TODO //TODO: Type
    "text-decoration-skip-spaces" = text_decoration_skip_spaces: TODO //TODO: Type
    "text-decoration-style" = text_decoration_style: TODO //TODO: Type
    "text-decoration-thickness" = text_decoration_thickness: TODO //TODO: Type
    "text-decoration-trim" = text_decoration_trim: TODO //TODO: Type
    "text-edge" = text_edge: TODO //TODO: Type
    "text-emphasis" = text_emphasis: TODO //TODO: Type
    "text-emphasis-color" = text_emphasis_color: Color
    "text-emphasis-position" = text_emphasis_position: TODO //TODO: Type
    "text-emphasis-skip" = text_emphasis_skip: TODO //TODO: Type
    "text-emphasis-style" = text_emphasis_style: TODO //TODO: Type
    "text-group-align" = text_group_align: TODO //TODO: Type
    "text-indent" = text_indent: TODO //TODO: Type
    "text-justify" = text_justify: TODO //TODO: Type
    "text-orientation" = text_orientation: TODO //TODO: Type
    "text-overflow" = text_overflow: TODO //TODO: Type
    "text-shadow" = text_shadow: TODO //TODO: Type
    "text-space-collapse" = text_space_collapse: TODO //TODO: Type
    "text-space-trim" = text_space_trim: TODO //TODO: Type
    "text-spacing" = text_spacing: TODO //TODO: Type
    "text-transform" = text_transform: TODO //TODO: Type
    "text-underline-offset" = text_underline_offset: TODO //TODO: Type
    "text-underline-position" = text_underline_position: TODO //TODO: Type
    "text-wrap" = text_wrap: TODO //TODO: Type
    "top" = top: Length
    "transform" = transform: TODO //TODO: Type
    "transform-box" = transform_box: TODO //TODO: Type
    "transform-origin" = transform_origin: TODO //TODO: Type
    "transform-style" = transform_style: TODO //TODO: Type
    "transition" = transition: TODO //TODO: Type
    "transition-delay" = transition_delay: TODO //TODO: Type
    "transition-duration" = transition_duration: TODO //TODO: Type
    "transition-property" = transition_property: TODO //TODO: Type
    "transition-timing-function" = transition_timing_function: TODO //TODO: Type
    "translate" = translate: TODO //TODO: Type
    "unicode-bidi" = unicode_bidi: TODO //TODO: Type
    "user-select" = user_select: TODO //TODO: Type
    "vertical-align" = vertical_align: TODO //TODO: Type
    "visibility" = visibility: TODO //TODO: Type
    "voice-balance" = voice_balance: TODO //TODO: Type
    "voice-duration" = voice_duration: TODO //TODO: Type
    "voice-family" = voice_family: TODO //TODO: Type
    "voice-pitch" = voice_pitch: TODO //TODO: Type
    "voice-range" = voice_range: TODO //TODO: Type
    "voice-rate" = voice_rate: TODO //TODO: Type
    "voice-stress" = voice_stress: TODO //TODO: Type
    "voice-volume" = voice_volume: TODO //TODO: Type
    "volume" = volume: TODO //TODO: Type
    "white-space" = white_space: TODO //TODO: Type
    "widows" = widows: TODO //TODO: Type
    "width" = width: Length
    "will-change" = will_change: TODO //TODO: Type
    "word-boundary-detection" = word_boundary_detection: TODO //TODO: Type
    "word-boundary-expansion" = word_boundary_expansion: TODO //TODO: Type
    "word-break" = word_break: TODO //TODO: Type
    "word-spacing" = word_spacing: TODO //TODO: Type
    "word-wrap" = word_wrap: TODO //TODO: Type
    "wrap-after" = wrap_after: TODO //TODO: Type
    "wrap-before" = wrap_before: TODO //TODO: Type
    "wrap-flow" = wrap_flow: TODO //TODO: Type
    "wrap-inside" = wrap_inside: TODO //TODO: Type
    "wrap-through" = wrap_through: TODO //TODO: Type
    "writing-mode" = writing_mode: TODO //TODO: Type
    "z-index" = z_index: TODO //TODO: Type
}
