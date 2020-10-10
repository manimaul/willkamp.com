use yew::{Component, ComponentLink, Html};
use yew::prelude::*;
use crate::bootstrap::traits::ComponentDemo;

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

impl ComponentDemo for Badge {
    fn demo() -> Vec<Badge> {
        vec![
            Badge {
                props: BadgeProps {
                    label: "Default".to_string(),
                    badge_type: BadgeType::Default
                }
            },
            Badge {
                props: BadgeProps {
                    label: "Primary".to_string(),
                    badge_type: BadgeType::Primary
                }
            },
            Badge {
                props: BadgeProps {
                    label: "Success".to_string(),
                    badge_type: BadgeType::Success
                }
            },
            Badge {
                props: BadgeProps {
                    label: "Info".to_string(),
                    badge_type: BadgeType::Info
                }
            },
            Badge {
                props: BadgeProps {
                    label: "Warning".to_string(),
                    badge_type: BadgeType::Warning
                }
            },
            Badge {
                props: BadgeProps {
                    label: "Danger".to_string(),
                    badge_type: BadgeType::Danger
                }
            },
        ]
    }
}