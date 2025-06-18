use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn Panel() -> Element {
    rsx! {
        div { id: "buttons",
            button {
                onclick: move |event| tracing::debug!{"Clicked skip button"},
                id: "skip", "skip"
            }
            button { onclick: move |event| tracing::debug!{"Clicked Event {event:?}"}, id: "save", "save!" }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Title {}
        Panel {}
    }
}
