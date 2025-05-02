use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SwitchPanelProps {
    show: Option<bool>,
    hidden_classes: Option<String>,
    hidden_content: Element,
    show_classes: Option<String>,
    show_content: Element,
}

#[component]
pub fn SwitchPanel(props: SwitchPanelProps) -> Element {
    let show = props.show.unwrap_or(false);

    if show {
        let classes = props.show_classes.unwrap_or_default();
        rsx! {
            div {
                class: "{classes}",
                {props.show_content}
            }
        }
    } else {
        let classes = props.hidden_classes.unwrap_or_default();
        rsx! {
            div {
                class: "{classes}",
                {props.hidden_content},
            }
        }
    }
}
