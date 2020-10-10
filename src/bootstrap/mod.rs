use yew::{Html, Component};
use yew::prelude::*;

mod badge;
mod planetarium;
mod traits;
mod button;

pub type Badge = badge::Badge;
pub type BadgeProps = badge::BadgeProps;
pub type BadgeType = badge::BadgeType;

pub type Button = button::Button;
pub type ButtonProps = button::ButtonProps;
pub type ButtonSize = button::ButtonSize;
pub type ButtonType = button::ButtonType;

pub type Planetarium = planetarium::Planetarium;
