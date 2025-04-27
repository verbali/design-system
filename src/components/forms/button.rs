use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone)]
enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ButtonLink<T>
where
    T: Routable,
{
    Internal(T),
    External(String),
}

impl<T> From<T> for ButtonLink<T>
where
    T: Routable,
{
    fn from(route: T) -> Self {
        ButtonLink::Internal(route)
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps<T>
where
    T: Routable + PartialEq,
{
    onclick: Option<EventHandler<MouseEvent>>,
    label: String,
    size: Option<ButtonSize>,
    link: Option<ButtonLink<T>>,
    r#type: Option<String>,
}

#[component]
pub fn Button<T>(props: ButtonProps<T>) -> Element
where
    T: Routable + PartialEq,
{
    let class =
        "inline-block bg-blue-700 dark:bg-blue-800 hover:bg-blue-800 dark:hover:bg-blue-700 font-bold text-white font-fredoka rounded-lg cursor-pointer";
    let classSize: String;

    match props.size {
        Some(size) => match size {
            ButtonSize::Small => classSize = "text-sm py-2 px-4".to_string(),
            ButtonSize::Medium => classSize = "text-base py-3 px-6".to_string(),
            ButtonSize::Large => classSize = "text-xl py-4 px-8".to_string(),
        },
        None => classSize = "text-base py-3 px-6".to_string(),
    }

    match props.link {
        Some(link) => match link {
            ButtonLink::Internal(internal_route) => {
                rsx! {
                    Link {
                        onclick: move |evt| {
                            if let Some(clickable) = props.onclick {
                                clickable.call(evt)
                            }
                        },
                        class: "{class} {classSize}",
                        to: internal_route,
                        "{props.label}"
                    }
                }
            }
            ButtonLink::External(external_link) => {
                rsx! {
                    a {
                        href: external_link,
                        class: "{class} {classSize}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "{props.label}"
                    }
                }
            }
        },
        None => {
            rsx! {
                button {
                    onclick: move |evt| {
                        if let Some(clickable) = props.onclick {
                            clickable.call(evt)
                        }
                    },
                    class: "{class} {classSize}",
                    type: props.r#type.unwrap_or("button".to_string()),
                    "{props.label}"
                }
            }
        }
    }
}
