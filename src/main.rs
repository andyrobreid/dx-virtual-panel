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

fn ConfigureState() -> Result<Vec<SignalKind>, &'static str> {
    //TODO Get configuration of state from a config yaml file
    Ok(vec![
        SignalKind::Digital(DigitalSignal {
            base: SignalProps {
                name: String::from("Dig Input 1"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: true,
        }),
        SignalKind::Digital(DigitalSignal {
            base: SignalProps {
                name: String::from("Dig Input 2"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: false,
        }),
        SignalKind::Analog(AnalogSignal {
            base: SignalProps {
                name: String::from("Ana Input 1"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: 100,
        }),
        SignalKind::Analog(AnalogSignal {
            base: SignalProps {
                name: String::from("Ana Input 1"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: 300,
        }),
    ])
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
    //TODO Handle scroll as an input to increase and decrease value live
    rsx!(
        input { title: "{props.base.tooltip}", type: "number", placeholder: "{props.base.name}" }
    )
}

#[component]
fn Panel() -> Element {
    let mut state = use_context::<State>();
    let signals_rendered = state.signals.iter().map(|sig| match sig {
        SignalKind::Digital(dig) => {
            rsx!(DigitalSignal {
                base: dig.base.clone(),
                value: dig.value
            })
        }
        SignalKind::Analog(ana) => {
            rsx!(AnalogSignal {
                base: ana.base.clone(),
                value: ana.value
            })
        }
    });
    rsx! {
        div { id: "buttons",
            button {
                onclick: move |event| tracing::debug!{"Clicked skip button with {event:?}"},
                id: "skip", "skip"
            },
            button { onclick: move |event| tracing::debug!{"Clicked Event {event:?}"}, id: "save", "save!" },
        }
        div {
            id: "inputs",
            {signals_rendered}
        }
    }
}

#[component]
fn App() -> Element {
    use_context_provider(|| State {
        signals: ConfigureState().unwrap(),
    });

    rsx! {
        Title {}
        Panel {}
    }
}

fn main() {
    dioxus::launch(App);
}
