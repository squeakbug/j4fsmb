#![allow(non_snake_case)]

pub mod home;
pub mod folder;
pub mod starred;
pub mod trash;
pub mod shared;
pub mod root;

use dioxus::prelude::*;

use folder::Folder;
use root::Root;
use home::Home;
use starred::Starred;
use trash::Trash;
use shared::Shared;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/home")]
    Home {},
    #[route("/root")]
    Root { },
    #[route("/folders/:id")]
    Folder { id: i32 },
    #[route("/starred")]
    Starred { },
    #[route("/trash")]
    Trash { },
    #[route("/shared")]
    Shared { },
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
