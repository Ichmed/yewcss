use crate::css_enum;

use super::{Length, IntoCss};

#[derive(Clone)]
pub enum BorderWidth {
    Thin,
    Medium,
    Thick,
    Custom(Length)
}

impl IntoCss for BorderWidth {
    fn into_css(self) -> String {
        match self {
            Self::Thin => "thin".to_string(),
            Self::Medium =>  "medium".to_string(),
            Self::Thick =>  "thick".to_string(),
            Self::Custom(size) => size.into_css()
        }
    }
}

impl From<Length> for BorderWidth {
    fn from(s: Length) -> Self {
        Self::Custom(s)
    }
}

impl From<i64> for BorderWidth {
    fn from(s: i64) -> Self {
        Self::Custom(s.into())
    }
}

css_enum!(BorderStyle; None | Hidden | Dotted | Dashed | Solid | Double | Groove | Ridge | Inset | Outset);
