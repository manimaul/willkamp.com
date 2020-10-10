use yew::Html;
use crate::components::PageHeader;
use yew::prelude::*;
use crate::routes::AppRoute;
use crate::components::Footer;

pub struct WaymakerPage {}
pub struct SoftwarePage {}
pub struct MarineElectronicsPage {}
pub struct HomePage {}
pub struct AboutPage {}

impl WaymakerPage {

    pub fn html() -> Html {
        html!{
            <div>
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
                { "Software"}
            </div>
        }
    }
}

impl MarineElectronicsPage {
    pub fn html() -> Html {
        html! {
            <div>
                { "MarineElectronics" }
            </div>
        }
    }
}
impl HomePage {
    pub fn html() -> Html {
        html! {
            <div>
                { "Home" }
            </div>
        }
    }
}
impl AboutPage {
    pub fn html() -> Html {
        html! {
            <div>
                { "About" }
            </div>
        }
    }
}
