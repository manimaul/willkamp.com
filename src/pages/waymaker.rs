use yew::Html;
use crate::components::nav_bar::NavBar;
use crate::components::hero::Hero;
use crate::components::page_header::PageHeader;
use yew::prelude::*;
use crate::routes::AppRoute;

pub struct WaymakerPage {}

impl WaymakerPage {
    pub fn html() -> Html {
        html!{
            <div>
                <NavBar active={AppRoute::Waymaker} />
                <Hero />
                    <PageHeader title={"SV Waymaker"} sub_title={"Tartan 37 Hull #144"}>
                        {"Sparkman & Stephens"}<br />
                        {"Deep Keel Model 6'7"}<br />
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