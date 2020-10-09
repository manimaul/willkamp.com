use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BadgeType {
    Default,
    Primary,
    Success,
    Info,
    Warning,
    Danger,
}

impl BadgeType {
    fn class(&self) -> &str {
        match self {
            BadgeType::Default => "badge badge-default",
            BadgeType::Primary => "badge badge-primary",
            BadgeType::Success => "badge badge-success",
            BadgeType::Info => "badge badge-info",
            BadgeType::Warning => "badge badge-warning",
            BadgeType::Danger => "badge badge-danger",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct BadgeProps {
    pub label: String,
    pub badge_type: BadgeType
}

pub struct Badge {
    props: BadgeProps
}

impl Component for Badge {
    type Message = ();
    type Properties = BadgeProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props == self.props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <span class={self.props.badge_type.class()}>{self.props.label.as_str()}</span>
        }
    }
}