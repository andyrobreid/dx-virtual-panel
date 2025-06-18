use dioxus::prelude::*;

static ICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Title {}
        DogView {}
    }
}

#[component]
fn DogView() -> Element {
    rsx! {
        div { id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "buttons",
            button {
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
