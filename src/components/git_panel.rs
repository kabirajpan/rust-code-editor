use dioxus::prelude::*;

#[component]
pub fn GitPanel() -> Element {
    rsx! {
        div {
            style: "padding: 15px; color: #cccccc;",
            h3 {
                style: "font-size: 0.85rem; font-weight: 600; color: #cccccc; margin: 0 0 15px 0; text-transform: uppercase; letter-spacing: 0.5px;",
                "Source Control"
            }

            // Commit message area
            div {
                style: "margin-bottom: 15px;",
                textarea {
                    style: "width: 100%; height: 80px; background-color: #1e1e1e; color: #cccccc; border: 1px solid #3c3c3c; border-radius: 4px; padding: 8px; font-size: 0.85rem; resize: none;",
                    placeholder: "Commit message..."
                }
            }

            // Commit button
            button {
                style: "width: 100%; background-color: #0e639c; color: white; border: none; padding: 8px; border-radius: 4px; cursor: pointer; font-size: 0.85rem; margin-bottom: 15px;",
                "✓ Commit"
            }

            // Changes section
            div {
                style: "margin-top: 15px;",
                div {
                    style: "font-size: 0.8rem; font-weight: 600; color: #858585; margin-bottom: 8px;",
                    "CHANGES (3)"
                }
                div {
                    style: "font-size: 0.85rem; margin-left: 10px;",
                    "M main.rs"
                    br {}
                    "A git_panel.rs"
                    br {}
                    "M layout/mod.rs"
                }
            }
        }
    }
}
