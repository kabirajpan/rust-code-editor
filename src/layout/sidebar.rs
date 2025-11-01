use crate::components::file_explorer::FileExplorer;
use crate::components::git_panel::GitPanel;
use crate::layout::icon_strip::PanelType;
use crate::layout::OpenFile;
use crate::theme::use_theme;
use dioxus::prelude::*;
#[component]
pub fn Sidebar(
    active_panel: Signal<Option<PanelType>>,
    open_files: Signal<Vec<OpenFile>>,
    active_file_index: Signal<Option<usize>>,
    workspace_path: Signal<String>, // ADD THIS
) -> Element {
    let colors = use_theme().colors();
    rsx! {
        aside {
            style: "flex: 1; background-color: {colors.bg_tertiary}; border-right: 1px solid {colors.border_primary}; display: flex; flex-direction: column; overflow: hidden; height: 100%;",
            {
                match active_panel() {
                    Some(PanelType::Files) => rsx! {
                        FileExplorer {
                            open_files: open_files,
                            active_file_index: active_file_index,
                            workspace_path: workspace_path, // ADD THIS
                        }
                    },
                    Some(PanelType::Search) => rsx! {
                        div {
                            style: "padding: 15px; color: {colors.text_primary};",
                            h3 { style: "font-size: 0.85rem; margin-bottom: 10px;", "Search" }
                            "Search functionality coming soon..."
                        }
                    },
                    Some(PanelType::Git) => rsx! { GitPanel {} },
                    Some(PanelType::Settings) => rsx! {
                        div {
                            style: "padding: 15px; color: {colors.text_primary};",
                            h3 { style: "font-size: 0.85rem; margin-bottom: 10px;", "Settings" }
                            "Settings panel coming soon..."
                        }
                    },
                    None => rsx! { div {} }
                }
            }
        }
    }
}