#![allow(non_snake_case)]

use dioxus::prelude::*;

pub use crate::components::Route;

#[component]
pub fn Shared() -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
    }
}