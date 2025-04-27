use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct LinkIconProps {
    size: Option<String>,
    color: Option<String>,
    class: Option<String>,
}

#[component]
pub fn LinkIcon(props: LinkIconProps) -> Element {
    let size = props.size.unwrap_or("36px".to_string());
    let color = props.color.unwrap_or("currentColor".to_string());
    let classes = props.class.unwrap_or_default();

    rsx! {
        svg {
            class: classes,
            view_box: "0 0 24 24",
            width: "{size}",
            height: "{size}",
            fill: "none",
            path {
                stroke_width: "2",
                stroke_linecap: "round",
                stroke: "{color}",
                d: "M14 12C14 14.7614 11.7614 17 9 17H7C4.23858 17 2 14.7614 2 12C2 9.23858 4.23858 7 7 7H7.5M10 12C10 9.23858 12.2386 7 15 7H17C19.7614 7 22 9.23858 22 12C22 14.7614 19.7614 17 17 17H16.5"
            }
        }
    }
}
