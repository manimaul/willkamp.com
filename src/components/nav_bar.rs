use yew::prelude::*;
use crate::routes::{AppRoute, AppAnchor};

#[derive (Properties, Clone, PartialEq)]
pub struct NavBarProps {
    pub active: AppRoute
}

pub struct NavBar {
    props: NavBarProps
}

impl Component for NavBar {
    type Message = ();
    type Properties = NavBarProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NavBar { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let routes: Vec<Html> = AppRoute::nav_menu()
            .into_iter()
            .map(|r| self.nav_html(&r))
            .collect();
        html! {
            <nav class="navbar navbar-expand-lg fixed-top navbar-dark bg-dark">
             <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarToggler" aria-controls="navbarToggler" aria-expanded="false" aria-label="Toggle navigation">
               <span class="navbar-toggler-icon"></span>
             </button>
             <a class="navbar-brand" href="/">{"William B. Kamp"}</a>
             <div class="collapse navbar-collapse" id="navbarToggler">
               <ul class="navbar-nav mr-auto mt-2 mt-lg-0">
                 { routes }
               </ul>
             </div>
           </nav>
        }
    }

}

impl NavBar {
    fn nav_html(&self, route: &AppRoute) -> Html {
        let class = if &self.props.active == route {
            "nav-item active"
        } else {
            "nav-item"
        };
        let name = route.display_name();
        html! {
          <li class=class>
            <AppAnchor classes={"nav-link"} route={route}>
                { name }
            </AppAnchor>
          </li>
        }
    }
}