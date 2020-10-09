use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let style = "background-color: #f5f5f5; \
        padding-bottom: 1rem !important;\
        padding-top: 1rem !important;\
        ";
        html! {
            <footer style={style}>
                <div class={"container"}>
                <p class={"text-muted"}>{"Author: William B. Kamp"}<br />
                <a class={"text-muted"} href={"mailto:manimaul@gmail.com"}>{"manimaul@gmail.com"}</a></p>
                <span class={"text-muted"}>{"Built with ♥️ Rust 🦀 and Web Assembly 🕸"}</span>
                </div>
            </footer>
        }
    }
}