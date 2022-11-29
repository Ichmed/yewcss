use super::IntoCss;

#[derive(Clone, Copy)]
pub enum Length {
    Percent(f64),

    /// font size of the element
    Em(f64), 
    /// x-height of the element’s font
    Ex(f64), 
    /// width of the "0" (ZERO, U+0030) glyph in the element’s font
    Ch(f64), 
    /// font size of the root element
    Rem(f64), 
    /// 1% of viewport’s width
    Vw(f64), 
    /// 1% of viewport’s height
    Vh(f64), 
    /// 1% of viewport’s smaller dimension
    Vmin(f64), 
    /// 1% of viewport’s larger dimension
    Vmax(f64),

    /// centimeters;	1cm = 96px/2.54
    Cm(f64),
    /// millimeters;	1mm = 1/10th of 1cm
    Mm(f64),
    /// quarter-millimeters;	1Q = 1/40th of 1cm
    Q(f64),
    /// inches;	1in = 2.54cm = 96px
    In(f64),
    /// picas;	1pc = 1/6th of 1in
    Pc(f64),
    /// points;	1pt = 1/72th of 1in
    Pt(f64),
    /// pixels;	1px = 1/96th of 1in
    Px(f64),
}

impl IntoCss for Length {
    fn into_css(self) -> String {
        match self {
            Length::Percent(length) => format!("{length}%"),

            Length::Em(length) => format!("{length}em"),
            Length::Ex(length) => format!("{length}ex"),
            Length::Ch(length) => format!("{length}ch"),
            Length::Rem(length) => format!("{length}rem"),
            Length::Vw(length) => format!("{length}vw"),
            Length::Vh(length) => format!("{length}vh"),
            Length::Vmin(length) => format!("{length}vmin"),
            Length::Vmax(length) => format!("{length}vmax"),

            Length::Cm(length) => format!("{length}cm"),
            Length::Mm(length) => format!("{length}mm"),
            Length::Q(length) => format!("{length}q"),
            Length::In(length) => format!("{length}in"),
            Length::Pc(length) => format!("{length}pc"),
            Length::Pt(length) => format!("{length}pt"),
            Length::Px(length) => format!("{length}px"),
        }
    }
}

impl From<i64> for Length {
    fn from(size: i64) -> Self {
        Self::Px(size as f64)
    }
}

impl From<f64> for Length {
    fn from(size: f64) -> Self {
        Self::Percent(size * 100.0)
    }
}

#[derive(Clone, Copy)]
pub enum Angle {
    Percent(f64),
    
    /// Degrees. There are 360 degrees in a full circle.
    Deg(f64),
    /// Gradians, also known as "gons" or "grades". There are 400 gradians in a full circle.
    Grad(f64),
    /// Radians. There are 2π radians in a full circle.
    Rad(f64),
    /// Turns. There is 1 turn in a full circle.
    Turn(f64),
}

impl From<i64> for Angle {
    fn from(size: i64) -> Self {
        Self::Deg(size as f64)
    }
}

impl From<f64> for Angle {
    fn from(size: f64) -> Self {
        Self::Rad(size * 100.0)
    }
}

#[derive(Clone, Copy)]
pub enum Duration {
    Percent(f64),

    /// Seconds.
    S(f64),
    /// Milliseconds. There are 1000 milliseconds in a second.
    Ms(f64),
}


impl IntoCss for Duration {
    fn into_css(self) -> String {
        match self {
            Self::Percent(duration) => format!("{duration}%"),

            Self::S(duration) => format!("{duration}s"),
            Self::Ms(duration) => format!("{duration}ms"),
        }
    }
}


impl From<i64> for Duration {
    fn from(size: i64) -> Self {
        Self::Ms(size as f64)
    }
}

impl From<f64> for Duration {
    fn from(size: f64) -> Self {
        Self::S(size)
    }
}

#[derive(Clone, Copy)]
pub enum Frequency {
    Percent(f64),
    /// Hertz. It represents the number of occurrences per second.
    Hz(f64),
    /// KiloHertz. A kiloHertz is 1000 Hertz.
    KHz(f64),
}