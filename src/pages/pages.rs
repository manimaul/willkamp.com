use yew::Html;
use crate::components::nav_bar::NavBar;
use crate::components::hero::Hero;
use crate::components::page_header::PageHeader;
use yew::prelude::*;
use crate::routes::AppRoute;
use crate::components::footer::Footer;

pub struct WaymakerPage {}
pub struct SoftwarePage {}
pub struct MarineElectronicsPage {}
pub struct HomePage {}
pub struct AboutPage {}

impl WaymakerPage {
    pub fn html() -> Html {
        html!{
            <div>
                <NavBar active={AppRoute::Waymaker} />
                <Hero />
                    <PageHeader title={"SV Waymaker"} sub_title={"Tartan 37 Hull #144"}>
                        {"Designer: Sparkman & Stephens"}<br />
                        {"Deep Keel Model"}<br />
                        {"Draft: 6'7"}<br />
                        {"Height (to water): 52'"}<br />
                        {"Displacement: 15,500 lbs."}<br />
                        {"LOD: 37'3\""}<br />
                        {"LWL: 28'6\""}<br />
                        {"Beam: 11'9\""}<br />
                        <br />
                        <div class={"embed-responsive embed-responsive-16by9"}>
                            <iframe class={"embed-responsive-item"} src={"https://www.youtube.com/embed/sfst3um8v9c"}></iframe>
                        </div>
                </PageHeader>
            </div>
            }
    }
}

impl SoftwarePage {
    pub fn html() -> Html {
        html!{
            <div>
                <NavBar active={AppRoute::Software} />
                <Hero />
                { "Software"}
            </div>
            }
    }
}

impl MarineElectronicsPage {
    pub fn html() -> Html {
        html! {
            <div>
                <NavBar active={AppRoute::MarineElectronics} />
                <Hero />
                { "MarineElectronics" }
            </div>
        }
    }
}
impl HomePage {
    pub fn html() -> Html {
        html! {
            <div>
                <NavBar active={AppRoute::Home} />
                <Hero />
                { "Home" }
            </div>
        }
    }
}
impl AboutPage {
    pub fn html() -> Html {
        html! {
            <div>
                <NavBar active={AppRoute::About} />
                <Hero />
                { "About" }
            </div>
        }
    }
}
