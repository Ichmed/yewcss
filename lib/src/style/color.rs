use std::fmt::Display;

use super::IntoCss;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, f64),
    Named(NamedColor),
    Transparent
}

impl IntoCss for Color {
    fn into_css(self) -> String {
        match self {
            Self::RGB(r, g, b) => format!("rgb({r}, {g}, {b})"),
            Self::RGBA(r, g, b, a) => format!("rgba({r}, {g}, {b}, {a})"),
            Self::Named(name) => format!("{name}"),
            Self::Transparent => format!("transparent")
        }
    }
}

impl Display for NamedColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait ToRGB {
    fn to_rgb(self) -> Color;
}

trait ToRGBA {
    fn to_rgba(self) -> Color;
}

impl<T: ToRGB> ToRGBA for T {
    fn to_rgba(self) -> Color {
        match self.to_rgb() {
            Color::RGB(r, g, b) => Color::RGBA(r, g, b, 1.0),
            x => panic!("{}.to_rgb() returned {:?}", stringify!(T), x)
        }        
    }
}

macro_rules! impl_NamedColor {
    ($($name:ident rgb($r:literal, $g:literal, $b:literal)),*) => {
        #[derive(Clone, Copy, Debug)]
        pub enum NamedColor {
            $($name),*
        }

        impl ToRGB for NamedColor {
            fn to_rgb(self) -> Color {
                match self {
                    $(NamedColor::$name => Color::RGB($r, $g, $b)),*
                }
            }
        }
    };
}

impl From<NamedColor> for Color {
    fn from(color: NamedColor) -> Self {
        Self::Named(color)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self::RGB(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(u8, u8, u8, f64)> for Color {
    fn from(rgb: (u8, u8, u8, f64)) -> Self {
        Self::RGBA(rgb.0, rgb.1, rgb.2, rgb.3)
    }
}

impl_NamedColor! {
    Aliceblue	rgb(240, 248, 255),
    Antiquewhite	rgb(250, 235, 215),
    Aqua	rgb( 0, 255, 255),
    Aquamarine	rgb(127, 255, 212),
    Azure	rgb(240, 255, 255),
    Beige	rgb(245, 245, 220),
    Bisque	rgb(255, 228, 196),
    Black	rgb( 0, 0, 0),
    Blanchedalmond	rgb(255, 235, 205),
    Blue	rgb( 0, 0, 255),
    Blueviolet	rgb(138, 43, 226),
    Brown	rgb(165, 42, 42),
    Burlywood	rgb(222, 184, 135),
    Cadetblue	rgb( 95, 158, 160),
    Chartreuse	rgb(127, 255, 0),
    Chocolate	rgb(210, 105, 30),
    Coral	rgb(255, 127, 80),
    Cornflowerblue	rgb(100, 149, 237),
    Cornsilk	rgb(255, 248, 220),
    Crimson	rgb(220, 20, 60),
    Cyan	rgb( 0, 255, 255),
    Darkblue	rgb( 0, 0, 139),
    Darkcyan	rgb( 0, 139, 139),
    Darkgoldenrod	rgb(184, 134, 11),
    Darkgray	rgb(169, 169, 169),
    Darkgreen	rgb( 0, 100, 0),
    Darkgrey	rgb(169, 169, 169),
    Darkkhaki	rgb(189, 183, 107),
    Darkmagenta	rgb(139, 0, 139),
    Darkolivegreen	rgb( 85, 107, 47),
    Darkorange	rgb(255, 140, 0),
    Darkorchid	rgb(153, 50, 204),
    Darkred	rgb(139, 0, 0),
    Darksalmon	rgb(233, 150, 122),
    Darkseagreen	rgb(143, 188, 143),
    Darkslateblue	rgb( 72, 61, 139),
    Darkslategray	rgb( 47, 79, 79),
    Darkslategrey	rgb( 47, 79, 79),
    Darkturquoise	rgb( 0, 206, 209),
    Darkviolet	rgb(148, 0, 211),
    Deeppink	rgb(255, 20, 147),
    Deepskyblue	rgb( 0, 191, 255),
    Dimgray	rgb(105, 105, 105),
    Dimgrey	rgb(105, 105, 105),
    Dodgerblue	rgb( 30, 144, 255),
    Firebrick	rgb(178, 34, 34),
    Floralwhite	rgb(255, 250, 240),
    Forestgreen	rgb( 34, 139, 34),
    Fuchsia	rgb(255, 0, 255),
    Gainsboro	rgb(220, 220, 220),
    Ghostwhite	rgb(248, 248, 255),
    Gold	rgb(255, 215, 0),
    Goldenrod	rgb(218, 165, 32),
    Gray	rgb(128, 128, 128),
    Grey	rgb(128, 128, 128),
    Green	rgb( 0, 128, 0),
    Greenyellow	rgb(173, 255, 47),
    Honeydew	rgb(240, 255, 240),
    Hotpink	rgb(255, 105, 180),
    Indianred	rgb(205, 92, 92),
    Indigo	rgb( 75, 0, 130),
    Ivory	rgb(255, 255, 240),
    Khaki	rgb(240, 230, 140),
    Lavender	rgb(230, 230, 250),
    Lavenderblush	rgb(255, 240, 245),
    Lawngreen	rgb(124, 252, 0),
    Lemonchiffon	rgb(255, 250, 205),
    Lightblue	rgb(173, 216, 230),
    Lightcoral	rgb(240, 128, 128),
    Lightcyan	rgb(224, 255, 255),
    Lightgoldenrodyellow	rgb(250, 250, 210),
    Lightgray	rgb(211, 211, 211),
    Lightgreen	rgb(144, 238, 144),
    Lightgrey	rgb(211, 211, 211),
    Lightpink	rgb(255, 182, 193),
    Lightsalmon	rgb(255, 160, 122),
    Lightseagreen	rgb( 32, 178, 170),
    Lightskyblue	rgb(135, 206, 250),
    Lightslategray	rgb(119, 136, 153),
    Lightslategrey	rgb(119, 136, 153),
    Lightsteelblue	rgb(176, 196, 222),
    Lightyellow	rgb(255, 255, 224),
    Lime	rgb( 0, 255, 0),
    Limegreen	rgb( 50, 205, 50),
    Linen	rgb(250, 240, 230),
    Magenta	rgb(255, 0, 255),
    Maroon	rgb(128, 0, 0),
    Mediumaquamarine	rgb(102, 205, 170),
    Mediumblue	rgb( 0, 0, 205),
    Mediumorchid	rgb(186, 85, 211),
    Mediumpurple	rgb(147, 112, 219),
    Mediumseagreen	rgb( 60, 179, 113),
    Mediumslateblue	rgb(123, 104, 238),
    Mediumspringgreen	rgb( 0, 250, 154),
    Mediumturquoise	rgb( 72, 209, 204),
    Mediumvioletred	rgb(199, 21, 133),
    Midnightblue	rgb( 25, 25, 112),
    Mintcream	rgb(245, 255, 250),
    Mistyrose	rgb(255, 228, 225),
    Moccasin	rgb(255, 228, 181),
    Navajowhite	rgb(255, 222, 173),
    Navy	rgb( 0, 0, 128),
    Oldlace	rgb(253, 245, 230),
    Olive	rgb(128, 128, 0),
    Olivedrab	rgb(107, 142, 35),
    Orange	rgb(255, 165, 0),
    Orangered	rgb(255, 69, 0),
    Orchid	rgb(218, 112, 214),
    Palegoldenrod	rgb(238, 232, 170),
    Palegreen	rgb(152, 251, 152),
    Paleturquoise	rgb(175, 238, 238),
    Palevioletred	rgb(219, 112, 147),
    Papayawhip	rgb(255, 239, 213),
    Peachpuff	rgb(255, 218, 185),
    Peru	rgb(205, 133, 63),
    Pink	rgb(255, 192, 203),
    Plum	rgb(221, 160, 221),
    Powderblue	rgb(176, 224, 230),
    Purple	rgb(128, 0, 128),
    Red	rgb(255, 0, 0),
    Rosybrown	rgb(188, 143, 143),
    Royalblue	rgb( 65, 105, 225),
    Saddlebrown	rgb(139, 69, 19),
    Salmon	rgb(250, 128, 114),
    Sandybrown	rgb(244, 164, 96),
    Seagreen	rgb( 46, 139, 87),
    Seashell	rgb(255, 245, 238),
    Sienna	rgb(160, 82, 45),
    Silver	rgb(192, 192, 192),
    Skyblue	rgb(135, 206, 235),
    Slateblue	rgb(106, 90, 205),
    Slategray	rgb(112, 128, 144),
    Slategrey	rgb(112, 128, 144),
    Snow	rgb(255, 250, 250),
    Springgreen	rgb( 0, 255, 127),
    Steelblue	rgb( 70, 130, 180),
    Tan	rgb(210, 180, 140),
    Teal	rgb( 0, 128, 128),
    Thistle	rgb(216, 191, 216),
    Tomato	rgb(255, 99, 71),
    Turquoise	rgb( 64, 224, 208),
    Violet	rgb(238, 130, 238),
    Wheat	rgb(245, 222, 179),
    White	rgb(255, 255, 255),
    Whitesmoke	rgb(245, 245, 245),
    Yellow	rgb(255, 255, 0),
    Yellowgreen rgb(154, 205, 50)
}