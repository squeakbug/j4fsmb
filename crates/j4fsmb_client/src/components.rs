#![allow(non_snake_case)]

pub mod home;
pub mod blog;

use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
