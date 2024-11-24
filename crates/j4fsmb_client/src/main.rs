#![allow(non_snake_case)]

pub mod components;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use components::{
    App,
    blog::Blog,
    home::Home,
};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}