#![allow(non_snake_case)]

use dioxus::prelude::*;

pub use crate::components::Route;

#[component]
pub fn Folder(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Folder {id}"
    }
}