use super::{url::Url, IntoCss};

#[derive(Clone)]
pub enum Image {
    Url(Url),
    Gradient(Gradient)
}

impl IntoCss for Image {
    fn into_css(self) -> String {
        match self {
            Self::Url(u) => u.into_css(),
            Self::Gradient(g) => g.into_css()
        }
    }
}

impl From<url::Url> for Image {
    fn from(u: url::Url) -> Self {
        Self::Url(u.into())
    }
}

#[derive(Clone)]
pub enum Gradient {

}

impl IntoCss for Gradient {
    fn into_css(self) -> String {
        todo!()
    }
}