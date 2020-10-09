use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Primary
    }
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Large,
    Default,
    Small,
    Block,
}

impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize::Default
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    #[prop_or_default]
    pub button_type: ButtonType,
    #[prop_or_default]
    pub button_size: ButtonSize,
    #[prop_or(false)]
    pub is_active: bool,
    #[prop_or(false)]
    pub outline: bool,
}

impl ButtonProps {
    fn class(&self) -> String {
        let outline = if self.outline {
            "-outline"
        } else {
            ""
        };
        let active = if self.is_active {
            " active"
        } else {
            ""
        };
        let size = match self.button_size {
            ButtonSize::Large => " btn-lg",
            ButtonSize::Default => "",
            ButtonSize::Small => " btn-sm",
            ButtonSize::Block => " btn-block",
        };

        match self.button_type {
            ButtonType::Primary => format!("btn btn{}-primary{}{}", outline, size, active),
            ButtonType::Secondary => format!("btn btn{}-secondary{}{}", outline, size, active),
            ButtonType::Success => format!("btn btn{}-success{}{}", outline, size, active),
            ButtonType::Danger => format!("btn btn{}-danger{}{}", outline, size, active),
            ButtonType::Warning => format!("btn btn{}-warning{}{}", outline, size, active),
            ButtonType::Info => format!("btn btn{}-info{}{}", outline, size, active),
            ButtonType::Light => format!("btn btn{}-light{}{}", outline, size, active),
            ButtonType::Dark => format!("btn btn{}-dark{}{}", outline, size, active),
        }
    }
}

pub struct Button {
    props: ButtonProps
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

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
            <button type={"button"} class={self.props.class()}>{self.props.text.as_str()}</button>
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::button::{ButtonProps, ButtonType, ButtonSize};

    #[test]
    fn props_class() {
        let mut props = ButtonProps {
            text: "".to_string(),
            button_type: Default::default(),
            button_size: Default::default(),
            is_active: false,
            outline: false
        };

        assert_eq!("btn btn-primary".to_string(), props.class());

        props.button_type = ButtonType::Danger;
        assert_eq!("btn btn-danger".to_string(), props.class());

        props.is_active = true;
        assert_eq!("btn btn-danger active".to_string(), props.class());

        props.outline = true;
        assert_eq!("btn btn-outline-danger active".to_string(), props.class());

        props.button_size = ButtonSize::Large;
        assert_eq!("btn btn-outline-danger btn-lg active".to_string(), props.class());
    }
}