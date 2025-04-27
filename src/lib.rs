pub mod components;

pub mod assets {
    use dioxus::prelude::*;
    pub const LOGO_SVG: Asset = asset!("/assets/logo.svg");
    pub const DS_CSS: Asset = asset!("/assets/ds.css");

    pub mod fonts {
        use dioxus::prelude::*;
        pub const REGULAR: Asset = asset!("/assets/fonts/Fredoka-Regular.ttf");
        pub const BOLD: Asset = asset!("/assets/fonts/Fredoka-SemiBold.ttf");
    }
}
