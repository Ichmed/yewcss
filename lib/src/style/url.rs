use super::IntoCss;

pub type Url = url::Url;

impl IntoCss for Url {
    fn into_css(self) -> String {
        format!("url(\"{}\")", self.to_string())
    }
}