use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ClipboardIconProps {
    size: Option<String>,
    color: Option<String>,
    class: Option<String>,
}

#[component]
pub fn ClipboardIcon(props: ClipboardIconProps) -> Element {
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
                stroke_linejoin: "round",
                stroke: "{color}",
                d: "M8 5.00005C7.01165 5.00082 6.49359 5.01338 6.09202 5.21799C5.71569 5.40973 5.40973 5.71569 5.21799 6.09202C5 6.51984 5 7.07989 5 8.2V17.8C5 18.9201 5 19.4802 5.21799 19.908C5.40973 20.2843 5.71569 20.5903 6.09202 20.782C6.51984 21 7.07989 21 8.2 21H15.8C16.9201 21 17.4802 21 17.908 20.782C18.2843 20.5903 18.5903 20.2843 18.782 19.908C19 19.4802 19 18.9201 19 17.8V8.2C19 7.07989 19 6.51984 18.782 6.09202C18.5903 5.71569 18.2843 5.40973 17.908 5.21799C17.5064 5.01338 16.9884 5.00082 16 5.00005M8 5.00005V7H16V5.00005M8 5.00005V4.70711C8 4.25435 8.17986 3.82014 8.5 3.5C8.82014 3.17986 9.25435 3 9.70711 3H14.2929C14.7456 3 15.1799 3.17986 15.5 3.5C15.8201 3.82014 16 4.25435 16 4.70711V5.00005M12 15H9M15 11H9"
            }
        }
    }
}
