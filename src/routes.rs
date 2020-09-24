use yew_router::{prelude::*, switch::Permissive};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/!"] Home,
    #[to = "/waymaker"] Waymaker,
    #[to = "/software"] Software,
    #[to = "/emarine"] MarineElectronics,
    #[to = "/about"] About,
    #[to = "/page-not-found"] PageNotFound(Permissive<String>),
}

pub type AppRouter = Router<AppRoute>;
// pub type AppAnchor = RouterAnchor<AppRoute>;