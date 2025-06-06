use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct FormIconProps {
    size: Option<String>,
    color: Option<String>,
}

#[component]
pub fn FormIcon(props: FormIconProps) -> Element {
    let size = props.size.unwrap_or("36px".to_string());
    let color = props.color.unwrap_or("currentColor".to_string());

    rsx! {
        svg {
            view_box: "0 0 52 52",
            width: "{size}",
            height: "{size}",
            fill: "none",
            rect {
                fill: "none",
                height: "4.8",
                width: "27.2",
                rx: "1.6",
                x: "12.4",
                y: "26"
            },
            rect {
                fill: "none",
                height: "4.8",
                width: "24",
                rx: "1.6",
                x: "12.4",
                y: "35.6"
            },
            path {
                fill: "{color}",
                d: "m36.4 14.8h8.48a1.09 1.09 0 0 0 1.12-1.12 1 1 0 0 0 -.32-.8l-10.56-10.56a1 1 0 0 0 -.8-.32 1.09 1.09 0 0 0 -1.12 1.12v8.48a3.21 3.21 0 0 0 3.2 3.2z"
            },
            path {
                fill: "{color}",
                d: "m44.4 19.6h-11.2a4.81 4.81 0 0 1 -4.8-4.8v-11.2a1.6 1.6 0 0 0 -1.6-1.6h-16a4.81 4.81 0 0 0 -4.8 4.8v38.4a4.81 4.81 0 0 0 4.8 4.8h30.4a4.81 4.81 0 0 0 4.8-4.8v-24a1.6 1.6 0 0 0 -1.6-1.6zm-32-1.6a1.62 1.62 0 0 1 1.6-1.55h6.55a1.56 1.56 0 0 1 1.57 1.55v1.59a1.63 1.63 0 0 1 -1.59 1.58h-6.53a1.55 1.55 0 0 1 -1.58-1.58zm24 20.77a1.6 1.6 0 0 1 -1.6 1.6h-20.8a1.6 1.6 0 0 1 -1.6-1.6v-1.57a1.6 1.6 0 0 1 1.6-1.6h20.8a1.6 1.6 0 0 1 1.6 1.6zm3.2-9.6a1.6 1.6 0 0 1 -1.6 1.63h-24a1.6 1.6 0 0 1 -1.6-1.6v-1.6a1.6 1.6 0 0 1 1.6-1.6h24a1.6 1.6 0 0 1 1.6 1.6z"
            }
        }
    }
}
