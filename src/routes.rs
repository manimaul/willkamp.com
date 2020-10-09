use yew_router::{prelude::*, switch::Permissive};

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/!"] Home,
    #[to = "/waymaker"] Waymaker,
    #[to = "/software"] Software,
    #[to = "/emarine"] MarineElectronics,
    #[to = "/about"] About,
    #[to = "/page-not-found"] PageNotFound(Permissive<String>),
}

impl AppRoute {
    pub fn display_name(&self) -> &str {
        match self {
            AppRoute::Home => "Home",
            AppRoute::Waymaker => "SV Waymaker",
            AppRoute::Software => "Software",
            AppRoute::MarineElectronics => "Marine Electronics",
            AppRoute::About => "About Me",
            AppRoute::PageNotFound(_) => "Ruh Roh",
        }
    }

    pub fn nav_menu() -> Vec<Self> {
        return vec![AppRoute::Home,
                    AppRoute::Waymaker,
                    AppRoute::Software,
                    AppRoute::MarineElectronics,
                    AppRoute::About]
    }
}

pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;