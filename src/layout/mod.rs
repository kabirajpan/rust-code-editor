use crate::theme::use_theme;
use dioxus::prelude::*;
mod icon_strip;
mod main_content;
mod menu_bar;
mod sidebar;
mod tab_bar;

use icon_strip::{IconStrip, PanelType};
use main_content::MainContent;
use menu_bar::MenuBar;
use sidebar::Sidebar;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct OpenFile {
    pub path: PathBuf,
}

#[component]
pub fn Layout() -> Element {
    let mut strip_visible = use_signal(|| true);
    let mut active_panel = use_signal(|| Some(PanelType::Files));
    let mut sidebar_width = use_signal(|| 200.0);
    let mut is_resizing = use_signal(|| false);
    let mut terminal_visible = use_signal(|| false);
    let mut right_sidebar_visible = use_signal(|| false);
    let mut is_split_horizontal = use_signal(|| false);

    // Main state for open files
    let mut open_files = use_signal(|| Vec::<OpenFile>::new());
    let mut active_file_index = use_signal(|| None::<usize>);

    // SEPARATE state for the right split pane
    let mut right_pane_file_index = use_signal(|| None::<usize>);

    let colors = use_theme().colors();

    let mut workspace_path = use_signal(|| {
        std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    });

    // File operation handlers
    let on_open_file = move |path: String| {
        let path_buf = PathBuf::from(path);

        let mut files = open_files.write();
        if let Some(existing_index) = files.iter().position(|f| f.path == path_buf) {
            active_file_index.set(Some(existing_index));
        } else {
            files.push(OpenFile { path: path_buf });
            active_file_index.set(Some(files.len() - 1));
        }
    };

    let on_open_folder = move |path: String| {
        println!("Setting workspace to: {}", path);
        workspace_path.set(path);
        open_files.write().clear();
        active_file_index.set(None);
        right_pane_file_index.set(None);
    };

    let on_new_file = move |_: ()| {
        println!("Creating new file");
    };

    let on_save_file = move |_: ()| {
        println!("Saving current file");
    };

    let on_save_as = move |_: ()| {
        println!("Save as dialog");
    };

    rsx! {
        div {
            style: "height: 100vh; width: 100vw; display: flex; flex-direction: column; overflow: hidden; background-color: {colors.bg_primary};",
            onmousemove: move |evt| {
                if is_resizing() {
                    let new_width = evt.client_coordinates().x - 40.0;
                    if new_width >= 150.0 && new_width <= 600.0 {
                        sidebar_width.set(new_width);
                    }
                }
            },
            onmouseup: move |_| {
                is_resizing.set(false);
            },
            MenuBar {
                strip_visible: strip_visible,
                on_toggle_strip: move |_| {
                    strip_visible.set(!strip_visible());
                    if !strip_visible() {
                        active_panel.set(None);
                    } else {
                        active_panel.set(Some(PanelType::Files));
                    }
                },
                terminal_visible: terminal_visible,
                on_toggle_terminal: move |_| {
                    terminal_visible.set(!terminal_visible());
                },
                right_sidebar_visible: right_sidebar_visible,
                on_toggle_right_sidebar: move |_| {
                    right_sidebar_visible.set(!right_sidebar_visible());
                },
                on_open_file: EventHandler::new(on_open_file),
                on_open_folder: EventHandler::new(on_open_folder),
                on_new_file: EventHandler::new(on_new_file),
                on_save_file: EventHandler::new(on_save_file),
                on_save_as: EventHandler::new(on_save_as),
            }
            div {
                style: "flex: 1; display: flex; flex-direction: row; position: relative; overflow: visible; min-height: 0; height: calc(100vh - 30px);",
                if strip_visible() {
                    IconStrip {
                        active_panel: active_panel,
                        on_panel_change: move |panel| {
                            if active_panel() == Some(panel) {
                                active_panel.set(None);
                            } else {
                                active_panel.set(Some(panel));
                            }
                        }
                    }
                }
                if active_panel().is_some() {
                    div {
                        style: "width: {sidebar_width()}px; display: flex; position: relative; height: 100%;",
                        Sidebar {
                            active_panel: active_panel,
                            open_files: open_files,
                            active_file_index: active_file_index,
                            workspace_path: workspace_path,
                        }
                        div {
                            style: "width: 4px; background-color: transparent; cursor: col-resize; position: absolute; right: 0; top: 0; bottom: 0; z-index: 100;",
                            onmousedown: move |evt| {
                                evt.stop_propagation();
                                is_resizing.set(true);
                            },
                            onmouseenter: move |evt| {
                                evt.stop_propagation();
                            },
                            div {
                                style: "width: 1px; height: 100%; background-color: #007acc; margin-left: 1.5px; opacity: 0; pointer-events: none;",
                            }
                        }
                    }
                }
                div {
                    style: "flex: 1; display: flex; flex-direction: column; min-width: 0; height: 100%; min-height: 0; overflow: hidden;",

                    // Main content area
                    if !is_split_horizontal() {
                        div {
                            style: if terminal_visible() { "flex: 1; display: flex; flex-direction: column; min-height: 0;" } else { "flex: 1; display: flex; flex-direction: column; height: 100%;" },
                            MainContent {
                                open_files: open_files,
                                active_file_index: active_file_index,
                                workspace_path: workspace_path,
                                on_split_right: Some(EventHandler::new(move |_| {
                                    is_split_horizontal.set(true);
                                    // Initialize right pane with the current active file
                                    right_pane_file_index.set(active_file_index());
                                })),
                                on_split_down: Some(EventHandler::new(move |_| {
                                    is_split_horizontal.set(true);
                                    right_pane_file_index.set(active_file_index());
                                })),
                                on_close_split: Some(EventHandler::new(move |_| {
                                    is_split_horizontal.set(false);
                                })),
                                is_split: Some(false),
                            }
                        }
                    } else {
                        // Split horizontally into two editors
                        div {
                            style: "flex: 1; display: flex; flex-direction: row; min-width: 0;",
                            // Left pane
                            div {
                                style: "flex: 1; min-width: 0; display: flex; flex-direction: column; border-right: 1px solid #3e3e42;",
                                MainContent {
                                    open_files: open_files,
                                    active_file_index: active_file_index,
                                    workspace_path: workspace_path,
                                    on_split_right: Some(EventHandler::new(move |_| {})),
                                    on_split_down: Some(EventHandler::new(move |_| {})),
                                    on_close_split: Some(EventHandler::new(move |_| {
                                        is_split_horizontal.set(false);
                                    })),
                                    is_split: Some(true),
                                }
                            }
                            // Right pane - uses its own file index
                            div {
                                style: "flex: 1; min-width: 0; display: flex; flex-direction: column;",
                                MainContent {
                                    open_files: open_files,
                                    active_file_index: right_pane_file_index,
                                    workspace_path: workspace_path,
                                    on_split_right: Some(EventHandler::new(move |_| {})),
                                    on_split_down: Some(EventHandler::new(move |_| {})),
                                    on_close_split: Some(EventHandler::new(move |_| {
                                        is_split_horizontal.set(false);
                                    })),
                                    is_split: Some(true),
                                }
                            }
                        }
                    }

                    // Terminal panel at bottom
                    if terminal_visible() {
                        Terminal {}
                    }
                }

                // Right sidebar
                if right_sidebar_visible() {
                    div {
                        style: "width: 300px; background-color: {colors.bg_tertiary}; border-left: 1px solid {colors.border_primary}; display: flex; flex-direction: column;",
                        div {
                            style: "padding: 15px; color: {colors.text_primary};",
                            h3 { style: "font-size: 0.85rem; margin-bottom: 10px;", "Right Panel" }
                            "Right sidebar functionality coming soon..."
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Terminal() -> Element {
    let mut terminal_input = use_signal(|| String::new());
    let mut terminal_output = use_signal(|| Vec::<String>::new());
    let colors = use_theme().colors();

    use_effect(move || {
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());
        terminal_output
            .write()
            .push(format!("Terminal started in: {}", cwd));
    });

    let execute_command = move |cmd: String| {
        spawn(async move {
            let mut output = terminal_output.write();
            output.push(format!("$ {}", cmd));

            if cmd.trim() == "clear" {
                output.clear();
                return;
            }

            match tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .current_dir(std::env::current_dir().unwrap_or_default())
                .output()
                .await
            {
                Ok(result) => {
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    let stderr = String::from_utf8_lossy(&result.stderr);

                    if !stdout.is_empty() {
                        for line in stdout.lines() {
                            output.push(line.to_string());
                        }
                    }

                    if !stderr.is_empty() {
                        for line in stderr.lines() {
                            output.push(format!("ERROR: {}", line));
                        }
                    }

                    if !result.status.success() {
                        if let Some(code) = result.status.code() {
                            output.push(format!("Process exited with code: {}", code));
                        }
                    }
                }
                Err(e) => {
                    output.push(format!("Failed to execute command: {}", e));
                }
            }
        });
    };

    rsx! {
        div {
            style: "height: 200px; background-color: {colors.bg_primary}; border-top: 1px solid {colors.border_primary}; display: flex; flex-direction: column; flex-shrink: 0;",

            div {
                style: "height: 30px; background-color: {colors.bg_secondary}; display: flex; align-items: center; justify-content: space-between; padding: 0 10px; border-bottom: 1px solid {colors.border_primary};",
                span {
                    style: "color: {colors.text_primary}; font-size: 0.85rem; font-weight: 500;",
                    "Terminal"
                }
                div {
                    style: "display: flex; gap: 4px;",
                    button {
                        style: "background: none; border: none; color: {colors.text_muted}; cursor: pointer; padding: 2px 4px; font-size: 10px;",
                        onclick: move |_| {
                            terminal_output.write().clear();
                        },
                        title: "Clear terminal",
                        "Clear"
                    }
                }
            }

            div {
                style: "flex: 1; padding: 10px; font-family: 'Consolas', 'Monaco', 'Courier New', monospace; font-size: 12px; color: {colors.text_primary}; overflow-y: auto; white-space: pre-wrap;",

                for (index, output_line) in terminal_output.read().iter().enumerate() {
                    div {
                        key: "{index}",
                        style: {
                            if output_line.starts_with("ERROR:") {
                                format!("margin-bottom: 2px; color: {};", colors.error)
                            } else if output_line.starts_with("$ ") {
                                format!("margin-bottom: 2px; color: {}; font-weight: 500;", colors.accent)
                            } else {
                                format!("margin-bottom: 2px; color: {};", colors.text_primary)
                            }
                        },
                        "{output_line}"
                    }
                }

                div {
                    style: "display: flex; align-items: center; gap: 4px; margin-top: 4px;",
                    span {
                        style: "color: {colors.accent}; font-weight: 500;",
                        "$ "
                    }
                    input {
                        style: "background: transparent; border: none; outline: none; color: {colors.text_primary}; font-family: inherit; font-size: inherit; flex: 1;",
                        r#type: "text",
                        value: terminal_input(),
                        placeholder: "Enter command...",
                        oninput: move |evt| terminal_input.set(evt.value()),
                        onkeypress: move |evt| {
                            if evt.key() == Key::Enter {
                                let input = terminal_input();
                                if !input.trim().is_empty() {
                                    execute_command(input);
                                }
                                terminal_input.set(String::new());
                            }
                        }
                    }
                }
            }
        }
    }
}
