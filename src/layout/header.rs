use dioxus::prelude::*;

#[component]
pub fn Header(on_toggle: EventHandler<()>) -> Element {
    rsx! {
        header {
            style: "height: 60px; background-color: #1e1e1e; display: flex; align-items: center; padding: 0 20px; border-bottom: 1px solid #333;",

            button {
                style: "background-color: #2d2d2d; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-size: 1rem; margin-right: 20px;",
                onclick: move |_| on_toggle.call(()),
                "☰ Toggle"
            }

            h1 {
                style: "font-size: 1.2rem; font-weight: bold; color: #e0e0e0; margin: 0;",
                "Code Editor IDE"
            }
        }
    }
}
