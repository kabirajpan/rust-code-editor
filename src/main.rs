use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

mod components;
mod editor;
mod layout;
mod theme;
mod utils;

use layout::Layout;
use theme::provide_theme_context;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    let config = Config::new().with_menu(None).with_window(
        WindowBuilder::new()
            .with_title("Code Editor IDE")
            .with_decorations(false),
    );

    LaunchBuilder::desktop().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    // Initialize theme context
    provide_theme_context();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Style {
            r#"
            * {{
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }}
            body {{
                margin: 0;
                padding: 0;
                overflow: hidden;
                user-select: none;
                -webkit-user-select: none;
                -moz-user-select: none;
                -ms-user-select: none;
            }}
            "#
        }
        Layout {}
    }
}
