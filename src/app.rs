use dioxus::prelude::*;
use crate::route::Route;

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}