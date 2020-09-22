#![recursion_limit="1024"]
mod nav_bar;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use nav_bar::NavBar;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<NavBar>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
