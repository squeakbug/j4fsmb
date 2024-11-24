#![allow(non_snake_case)]

use dioxus::prelude::*;

pub use crate::Route;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}