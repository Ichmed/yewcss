use yew::prelude::*;
use yewcss::{css, style::{border::BorderStyle, color::NamedColor, url::Url, Float}};

struct Model {
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        
        // Play around with this
        let paragraph_style = css!(
            width: 0.4;
            height: 150 + 10;
            margin: 5;
            padding: 50;
            background_image: Url::parse("https://picsum.photos/200/300").unwrap();
            color: (0, 0, 255, 0.5);
            border: 5 BorderStyle::Dotted NamedColor::Red;
            border_radius: 50 50 50;
            float: Float::Left;
        );

        // Do not touch
        let stylesheet_style = css!{
            float: Float::Right;
            margin_right: 0.25;
        };

        html! {
            <>
            <p style={paragraph_style.clone()}>{"I am a paragraph"}</p>
            <div style={stylesheet_style}>
                <p>{"{"}</p>
                {
                    for paragraph_style.to_string()
                    .split_inclusive(";")
                    .map(|x| html!(<p style={{css!{padding_left: 20;}}}>{x}</p>))
                }
                <p>{"}"}</p>
            </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
