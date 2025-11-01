use crate::components::file_tree::FileTree;
use crate::layout::OpenFile;
use crate::theme::use_theme;
use dioxus::prelude::*;

#[component]
pub fn FileExplorer(
    open_files: Signal<Vec<OpenFile>>,
    active_file_index: Signal<Option<usize>>,
    workspace_path: Signal<String>, // ADD THIS
) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; overflow: hidden;",
            div {
                style: {
                    let colors = use_theme().colors();
                    format!("padding: 5px 15px 5px 15px; flex-shrink: 0; border-bottom: 1px thin {};", colors.border_primary)
                },
                h3 {
                    style: {
                        let colors = use_theme().colors();
                        format!("font-size: 0.75rem; font-weight: 400; color: {}; margin: 0; text-transform: uppercase; letter-spacing: 0.5px;", colors.text_primary)
                    },
                    "Explorer"
                }
            }
            div {
                style: "flex: 1; overflow-y: auto; overflow-x: hidden; min-height: 0;",
                FileTree {
                    root_path: workspace_path(), // USE THE SIGNAL HERE
                    open_files: open_files,
                    active_file_index: active_file_index
                }
            }
        }
    }
}
