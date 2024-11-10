#![allow(non_snake_case)]

pub mod app;
pub mod route;


use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};



fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    #[cfg(target_arch = "wasm32")] 
    {
        launch(App);
    }
    #[cfg(not(target_arch = "wasm32"))] 
    {
        let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
        LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    }
    
}

