use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct SignalProps {
    name: String,
    tooltip: String,
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
fn DigitalSignal(props: SignalProps) -> Element {
    rsx!(
        button { title: "{props.tooltip}" , "{props.name}" }

    )
}

#[component]
fn Panel() -> Element {
    rsx! {
        div { id: "buttons",
            button {
                onclick: move |event| tracing::debug!{"Clicked skip button with {event:?}"},
                id: "skip", "skip"
            },
            button { onclick: move |event| tracing::debug!{"Clicked Event {event:?}"}, id: "save", "save!" },
            DigitalSignal{
                name: "TEST",
                tooltip: "this is a tooltip"
            }
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

fn main() {
    dioxus::launch(App);
}
