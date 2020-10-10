#![recursion_limit="1024"]

mod routes;
mod components;
mod pages;
mod bootstrap;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::components::Root;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
