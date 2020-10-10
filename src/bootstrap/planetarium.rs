use yew::{Component, ComponentLink, Html};
use yew::macros::html;
use std::sync::Mutex;
use crate::bootstrap::traits::ComponentDemo;
use crate::bootstrap::badge::Badge;

pub struct Planetarium {}

impl Component for Planetarium {
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
        let badge = Badge::demo_html();
        html! {
            <div>
                <h1>{"Planetarium Of Components"}</h1>
                <h6>{"bootstrap::Badge"}</h6>
                { badge }
                // <h6>{"bootstrap::Button"}</h6>
                // { button }
            </div>
        }
    }
}