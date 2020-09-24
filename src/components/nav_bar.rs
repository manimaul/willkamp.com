use yew::prelude::*;

pub struct NavBarLink {
    name: String,
    href: String,
}

impl NavBarLink {
    fn from(name: &str, href: &str) -> NavBarLink {
        return NavBarLink {
            name: String::from(name),
            href: String::from(href),
        };
    }

    fn a11y_screen_reader(&self, selected: bool) -> Html {
        match selected {
            true => html! { <span class="sr-only">{"(current)"}</span> },
            false => html! {}
        }
    }

    fn html(&self, selected: bool) -> Html {
        let class = match selected {
            true => "nav-item active",
            false => "nav-item"
        };

        html! {
          <li class=class>
            <a class="nav-link" href= {self.href.as_str()} >{self.name.as_str()} {self.a11y_screen_reader(selected)} </a>
          </li>
        }
    }
}

pub struct NavBar {}

impl Component for NavBar {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NavBar { }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
             <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarToggler" aria-controls="navbarToggler" aria-expanded="false" aria-label="Toggle navigation">
               <span class="navbar-toggler-icon"></span>
             </button>
             <a class="navbar-brand" href="/">{"William B. Kamp"}</a>
             <div class="collapse navbar-collapse" id="navbarToggler">
               <ul class="navbar-nav mr-auto mt-2 mt-lg-0">
                 { NavBarLink::from("SV Waymaker", "/waymaker").html(false) }
                 { NavBarLink::from("Software", "/software").html(false) }
                 { NavBarLink::from("Marine Electronics", "/emarine").html(false) }
                 { NavBarLink::from("About Me", "/about").html(false) }
               </ul>
             </div>
           </nav>
        }
    }
}