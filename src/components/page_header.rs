use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

#[derive (Properties, Clone, PartialEq)]
pub struct PageHeaderProps {
    pub children: Children,
    pub title: String,
    pub sub_title: String,
}

pub struct PageHeader {
    props: PageHeaderProps
}

impl Component for PageHeader {
    type Message = ();
    type Properties = PageHeaderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        return if props == self.props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class={"page-header"}>
                { self.title() }
                { self.props.children.clone() }
            </div>
        }
    }
}

impl PageHeader {

    fn sub_title(&self) -> Html {
        if self.props.sub_title.is_empty() {
            return html! {}
        } else {
            return html! {
             <small>{"\u{00a0}"}{self.props.sub_title.as_str()}</small>
            }
        };
    }

    fn title(&self) -> Html {
        html! {
            <h1>{ self.props.title.as_str() } { self.sub_title() }</h1>
        }
    }
}
