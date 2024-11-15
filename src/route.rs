use dioxus::prelude::*;


#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}



#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { 
                // class: "bg-lime-500	",
                "High-Five counter: {count}" 
            }
            button { 
                class: "bg-lime-500",
                onclick: move |_| count += 1, "Up high!",
            }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
