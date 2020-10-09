use yew::{Component, ComponentLink, Html};
use crate::routes::{AppRoute, AppRouter};
use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};
use crate::components::nav_bar::NavBar;
use crate::components::hero::Hero;

pub struct Root {}

impl Component for Root {
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
        let render = AppRouter::render(Root::switch);
        let redirect = AppRouter::redirect(|route: Route| {
            AppRoute::PageNotFound(Permissive(Some(route.route)))
        });
        html! {
            <div>
                <AppRouter render=render,redirect=redirect />
            </div>
        }
    }
}

impl Root {
    fn switch(route: AppRoute) -> Html {
        match route {
            AppRoute::Waymaker => html!{
                <div>
                    <NavBar active=route />
                    <Hero />
                    {"waymaker"}
                </div>
            },
            AppRoute::Software => html!{
                <div>
                    <NavBar active=route />
                    <Hero />
                    {"software"}
                </div>
            },
            AppRoute::MarineElectronics => html!{
                <div>
                    <NavBar active=route />
                    <Hero />
                    {"marine electronics"}
                </div>
            },
            AppRoute::Home  => html! {
                <div>
                    <NavBar active=route />
                    <Hero />
                    {"home"}
                </div>
            },
            AppRoute::About => html!{
                <div>
                    <NavBar active=route />
                    <Hero />
                    {"about"}
                </div>
            },
            AppRoute::PageNotFound(_) => html!{
                <div>
                    <div>{"ruh roh"}</div>
                </div>
            },
        }
    }
}