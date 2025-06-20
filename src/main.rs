use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct SignalProps {
    name: String,
    tooltip: String,
}

#[derive(Props, PartialEq, Clone)]
struct DigitalSignal {
    base: SignalProps,
    value: bool,
}

#[derive(Props, PartialEq, Clone)]
struct AnalogSignal {
    base: SignalProps,
    value: u32,
}

#[derive(Clone)]
enum SignalKind {
    Digital(DigitalSignal),
    Analog(AnalogSignal),
}

#[derive(Clone)]
struct State {
    signals: Vec<SignalKind>,
}

fn ConfigureState() -> Result<(), &'static str> {
    // Get configuration of state from a config yaml file
    todo!();
    Ok(())
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
fn DigitalSignal(props: DigitalSignal) -> Element {
    rsx!(
        button { title: "{props.base.tooltip}" , "{props.base.name}" }
    )
}

#[component]
fn AnalogSignal(props: AnalogSignal) -> Element {
    rsx!(
        input { title: "{props.base.tooltip}" , "{props.base.name}" }
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
                base: SignalProps {
                    name: String::from("TEST"),
                    tooltip: String::from("this is a Digial Input")
                },
                value: true
            },
            AnalogSignal{
                base: SignalProps { name: String::from("Test Analog"), tooltip: String::from("This is a analog input") },
                value: 100
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
