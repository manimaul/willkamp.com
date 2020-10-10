use yew::{Html, Component};
use yew::prelude::*;

mod badge;
mod planetarium;
mod traits;

pub type Badge = badge::Badge;
pub type BadgeProps = badge::BadgeProps;
pub type BadgeType = badge::BadgeType;

pub type Planetarium = planetarium::Planetarium;
