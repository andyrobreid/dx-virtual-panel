use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct SignalProps {
    name: String,
    tooltip: String,
}

#[derive(Props, PartialEq, Clone)]
struct DigitalSignalData {
    base: SignalProps,
    value: bool,
}

#[derive(Props, PartialEq, Clone)]
struct AnalogSignalData {
    base: SignalProps,
    value: u32,
}

#[derive(Clone)]
enum SignalKind {
    Digital(DigitalSignalData),
    Analog(AnalogSignalData),
}

#[derive(Clone)]
struct State {
    signals: Signal<Vec<SignalKind>>,
}

fn configure_state() -> Vec<SignalKind> {
    //TODO Get configuration of state from a config yaml file
    vec![
        SignalKind::Digital(DigitalSignalData {
            base: SignalProps {
                name: String::from("Dig Input 1"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: true,
        }),
        SignalKind::Digital(DigitalSignalData {
            base: SignalProps {
                name: String::from("Dig Input 2"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: false,
        }),
        SignalKind::Analog(AnalogSignalData {
            base: SignalProps {
                name: String::from("Ana Input 1"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: 100,
        }),
        SignalKind::Analog(AnalogSignalData {
            base: SignalProps {
                name: String::from("Ana Input 1"),
                tooltip: String::from("This is a Digital Input"),
            },
            value: 300,
        }),
    ]
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "Virtual-Panel" }
        }
    }
}

#[component]
fn DigitalSignal(props: DigitalSignalData) -> Element {
    let mut state = use_context::<State>();
    rsx!(
        button {
            title: "{props.base.tooltip}" ,
            onclick: move |_| {
                let mut signals = state.signals.write();
                if let Some(SignalKind::Digital(dig)) = signals.iter_mut().find(|s| match s {
                    SignalKind::Digital(dig) => dig.base.name == props.base.name,
                    _ => false
                }) {
                    dig.value = !dig.value;
                    tracing::info!("Button {:?} has value of {1}", props.base.name, props.value);
                }
            }
             ,
             "{props.base.name}" }
    )
}

#[component]
fn AnalogSignal(props: AnalogSignalData) -> Element {
    //TODO Handle scroll as an input to increase and decrease value live
    rsx!(
        input { title: "{props.base.tooltip}", type: "number", placeholder: "{props.base.name}" }
    )
}

#[component]    
fn Panel() -> Element {
    let state = use_context::<State>();
    let signals_ref = state.signals.read();
    let signals_rendered = signals_ref.iter().map(|sig| match sig {
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
        div {
            id: "inputs",
            {signals_rendered}
        }
    }
}

#[component]
fn App() -> Element {
    let signals = use_signal(|| configure_state());
    use_context_provider(|| State {
        signals,
    });

    rsx! {
        Title {}
        Panel {}
    }
}

fn main() {
    dioxus::launch(App);
}
