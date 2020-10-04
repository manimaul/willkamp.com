use yew::prelude::*;
use yew::{Component, ComponentLink, Html};

pub struct Hero {}

impl Component for Hero {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let style = "background-image: url(/images/hero_bg.jpg);\
         background-repeat: no-repeat;\
         background-position: center;\
         background-size: cover;\
         min-height: 200px";
        html! {
            <div style={style}>
            </div>
        }
    }
}