#![recursion_limit="1024"]

mod routes;
mod components;
mod pages;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::components::root::Root;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
