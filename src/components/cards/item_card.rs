use dioxus::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone)]
pub enum ItemCardIcon {
    Asset(Asset),
    Element(Element),
}

impl Display for ItemCardIcon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemCardIcon::Asset(asset) => write!(f, "{}", asset),
            ItemCardIcon::Element(_) => write!(f, "element"),
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ItemCardProps {
    icon: ItemCardIcon,
    title: String,
    content: String,
}

#[component]
pub fn ItemCard(props: ItemCardProps) -> Element {
    rsx! {
        div {
            class: "sm:flex sm:flex-row sm:items-center my-16",

            div {
                match props.icon {
                    ItemCardIcon::Asset(asset) => {
                        rsx!(img {
                            class: "w-16 h-16 mx-auto sm:mx-8",
                            src: "{asset}",
                            alt: "{props.title} icon"
                        })
                    },
                    ItemCardIcon::Element(element) => {
                        element
                    },
                }
            }

            div {
                h2 {
                    class: "font-bold text-4xl",
                    "{props.title}"
                }
                p {
                    class: "text-xl",
                    "{props.content}"
                }
            }
        }
    }
}
