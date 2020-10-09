use yew::{Component, ComponentLink, Html};
use crate::routes::{AppRoute, AppRouter};
use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};
use crate::components::nav_bar::NavBar;
use crate::components::hero::Hero;
use crate::components::page_header::PageHeader;
use crate::routes::AppRoute::{Waymaker, MarineElectronics};
use crate::pages;
use crate::pages::pages::{WaymakerPage, SoftwarePage, MarineElectronicsPage, HomePage, AboutPage};
use crate::components::footer::Footer;

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
        let render = AppRouter::render(Root::content);
        let redirect = AppRouter::redirect(|route: Route| {
            AppRoute::PageNotFound(Permissive(Some(route.route)))
        });
        html! {
            <main>
                <div>
                    <AppRouter render=render,redirect=redirect />
                </div>
                <Footer />
            </main>
        }
    }
}

impl Root {
    fn content(route: AppRoute) -> Html {
        let style = "padding-top: 16px;\
        padding-bottom: 16px";
        html! {
            <div>
                <NavBar active={route.clone()} />
                <Hero />
                <div class={"container"} style={style}>{Root::switch(route)}</div>
            </div>
        }
    }

    fn switch(route: AppRoute) -> Html {
        match route {
            AppRoute::Waymaker => WaymakerPage::html(),
            AppRoute::Software => SoftwarePage::html(),
            AppRoute::MarineElectronics => MarineElectronicsPage::html(),
            AppRoute::Home => HomePage::html(),
            AppRoute::About => AboutPage::html(),
            AppRoute::PageNotFound(_) => html! {
                <div>
                    <div>{"ruh roh"}</div>
                </div>
            },
        }
    }
}