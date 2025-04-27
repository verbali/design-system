use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct StatsIconProps {
    size: Option<String>,
    color: Option<String>,
}

#[component]
pub fn StatsIcon(props: StatsIconProps) -> Element {
    let size = props.size.unwrap_or("36px".to_string());
    let color = props.color.unwrap_or("currentColor".to_string());

    rsx! {
        svg {
            view_box: "-3 0 32 32",
            width: "{size}",
            height: "{size}",
            fill: "none",
            g {
                fill: "{color}",
                transform: "translate(-314.000000, -673.000000)",
                path {
                    d: "M328,673 L326,673 C324.896,673 324,673.896 324,675 L324,703 C324,704.104 324.896,705 326,705 L328,705 C329.104,705 330,704.104 330,703 L330,675 C330,673.896 329.104,673 328,673 L328,673 Z M338,689 L336,689 C334.896,689 334,689.896 334,691 L334,703 C334,704.104 334.896,705 336,705 L338,705 C339.104,705 340,704.104 340,703 L340,691 C340,689.896 339.104,689 338,689 L338,689 Z M318,682 L316,682 C314.896,682 314,682.896 314,684 L314,703 C314,704.104 314.896,705 316,705 L318,705 C319.104,705 320,704.104 320,703 L320,684 C320,682.896 319.104,682 318,682 L318,682 Z"
                }
            }
        }
    }
}
