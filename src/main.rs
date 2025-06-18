use dioxus::prelude::*;

static ICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Title {}
        Panel {}
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
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}
