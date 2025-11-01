use crate::editor::{RopeEditor, VirtualEditorView};
use crate::layout::tab_bar::TabBar;
use crate::layout::OpenFile;
use crate::theme::use_theme;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[component]
pub fn MainContent(
    open_files: Signal<Vec<OpenFile>>,
    active_file_index: Signal<Option<usize>>,
    workspace_path: Signal<String>,
    on_split_right: Option<EventHandler<()>>,
    on_split_down: Option<EventHandler<()>>,
    on_close_split: Option<EventHandler<()>>,
    is_split: Option<bool>,
) -> Element {
    let mut editors = use_signal(|| HashMap::<PathBuf, Signal<RopeEditor>>::new());

    // Clean up editors for closed files
    use_effect(move || {
        let files = open_files();
        let mut editors_map = editors.write();

        // Remove editors for files that are no longer open
        let open_paths: std::collections::HashSet<PathBuf> =
            files.iter().map(|f| f.path.clone()).collect();
        editors_map.retain(|path, _| open_paths.contains(path));
    });

    // Load editor for new files
    let _ = use_resource(move || {
        let files = open_files();
        async move {
            for file in files.iter() {
                let path = file.path.clone();

                if !editors.peek().contains_key(&path) {
                    let mut editor = RopeEditor::new();
                    if let Err(e) = editor.load_file(&path) {
                        eprintln!("Failed to load file {}: {}", path.display(), e);
                    } else {
                        let editor_signal = Signal::new(editor);
                        editors.write().insert(path, editor_signal);
                    }
                }
            }
        }
    });

    let handle_save = move |path: PathBuf| {
        if let Some(mut editor_signal) = editors.read().get(&path).cloned() {
            let mut editor = editor_signal.write();
            if let Err(e) = editor.save_file() {
                eprintln!("Failed to save file: {}", e);
            }
        }
    };

    // If no files are open, show welcome screen with TabBar
    if open_files.read().is_empty() {
        return rsx! {
            main {
                style: {
                    let colors = use_theme().colors();
                    format!(
                        "flex: 1; background-color: {}; display: flex; flex-direction: column; height: 100%; min-height: 100%; overflow: visible;",
                        colors.bg_primary
                    )
                },

                TabBar {
                    open_files: open_files,
                    active_file_index: active_file_index,
                    is_split: is_split.unwrap_or(false),
                    on_split_right: move |_| if let Some(cb) = &on_split_right { cb.call(()) },
                    on_split_down: move |_| if let Some(cb) = &on_split_down { cb.call(()) },
                    on_close_split: move |_| if let Some(cb) = &on_close_split { cb.call(()) },
                }

                div {
                    style: {
                        let colors = use_theme().colors();
                        format!(
                            "flex: 1; display: flex; align-items: center; justify-content: center;"
                        )
                    },
                    h2 {
                        style: {
                            let colors = use_theme().colors();
                            format!("font-size: 2rem; font-weight: 400; color: {}; margin: 0;", colors.text_muted)
                        },
                        "Start editing..."
                    }
                }
            }
        };
    }

    // Get the currently active file
    let active_file = active_file_index().and_then(|idx| open_files.read().get(idx).cloned());

    if let Some(file) = active_file {
        let editor_signal = editors.read().get(&file.path).cloned();

        // Build breadcrumb path relative to workspace root
        let workspace_path_buf = PathBuf::from(workspace_path());
        let workspace_name = workspace_path_buf
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Workspace")
            .to_string();

        let path_parts: Vec<String> =
            if let Ok(relative) = file.path.strip_prefix(&workspace_path_buf) {
                // File is inside workspace
                let mut parts = vec![workspace_name];

                // Add all path components from relative path
                for component in relative.components() {
                    if let Some(part) = component.as_os_str().to_str() {
                        if !part.is_empty() {
                            parts.push(part.to_string());
                        }
                    }
                }

                parts
            } else {
                // File is outside workspace, show full path
                file.path
                    .components()
                    .filter_map(|comp| comp.as_os_str().to_str())
                    .map(|s| s.to_string())
                    .collect()
            };

        let path_breadcrumb = path_parts.iter().enumerate().map(|(i, part)| {
            let is_last = i == path_parts.len() - 1;
            rsx! {
                span {
                    key: "{i}-{part}",
                    style: if is_last {
                        "color: #cccccc;"
                    } else {
                        "color: #858585;"
                    },
                    "{part}"
                }
                if !is_last {
                    span {
                        key: "{i}-separator",
                        style: "color: #505050; margin: 0 4px;",
                        "\u{203A}"
                    }
                }
            }
        });

        let editor_info = editor_signal.as_ref().map(|editor_sig| {
            let editor_read = editor_sig.read();
            let line_count = editor_read.line_count();
            let is_modified = editor_read.is_modified();
            drop(editor_read);

            rsx! {
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span {
                        style: "background-color: #4ec9b0; color: #1e1e1e; padding: 2px 8px; border-radius: 3px; font-size: 0.65rem; font-weight: 600;",
                        "ROPE EDITOR"
                    }
                    span {
                        style: "color: #858585; font-size: 0.7rem;",
                        "{line_count} lines"
                    }
                    if is_modified {
                        span {
                            style: "color: #d4a72c; font-size: 0.7rem;",
                            "\u{25CF} Modified"
                        }
                    }
                }
            }
        });

        let editor_content = if let Some(editor_sig) = editor_signal {
            let path_clone = file.path.clone();
            rsx! {
                VirtualEditorView {
                    key: "{file.path.to_string_lossy()}",
                    editor: editor_sig,
                    on_save: move |_| handle_save(path_clone.clone()),
                }
            }
        } else {
            rsx! {
                div {
                    style: "flex: 1; display: flex; align-items: center; justify-content: center;",
                    span {
                        style: "color: #858585;",
                        "Loading..."
                    }
                }
            }
        };

        rsx! {
            main {
                style: {
                    let colors = use_theme().colors();
                    format!(
                        "flex: 1; background-color: {}; display: flex; flex-direction: column; height: 100%; min-height: 100%; overflow: visible;",
                        colors.bg_primary
                    )
                },

                TabBar {
                    open_files: open_files,
                    active_file_index: active_file_index,
                    is_split: is_split.unwrap_or(false),
                    on_split_right: move |_| if let Some(cb) = &on_split_right { cb.call(()) },
                    on_split_down: move |_| if let Some(cb) = &on_split_down { cb.call(()) },
                    on_close_split: move |_| if let Some(cb) = &on_close_split { cb.call(()) },
                }

                div {
                    style: {
                        let colors = use_theme().colors();
                        format!(
                            "height: 22px; background-color: {}; border-bottom: 1px solid {}; display: flex; align-items: center; justify-content: space-between; padding: 0 15px; font-size: 0.75rem; color: {}; gap: 6px; flex-shrink: 0; position: relative; z-index: 2;",
                            colors.bg_primary,
                            colors.border_primary,
                            colors.text_muted
                        )
                    },
                    div {
                        style: "display: flex; align-items: center; gap: 6px;",
                        {path_breadcrumb}
                    }
                    {editor_info}
                }

                div {
                    key: "{file.path.to_string_lossy()}-container",
                    style: "flex: 1; display: flex; flex-direction: column; min-height: 0; overflow: hidden; height: calc(100% - 52px);",
                    {editor_content}
                }
            }
        }
    } else {
        rsx! {
            main {
                style: {
                    let colors = use_theme().colors();
                    format!(
                        "flex: 1; background-color: {}; display: flex; flex-direction: column; height: 100%; min-height: 100%; overflow: visible;",
                        colors.bg_primary
                    )
                },

                TabBar {
                    open_files: open_files,
                    active_file_index: active_file_index,
                    is_split: is_split.unwrap_or(false),
                    on_split_right: move |_| if let Some(cb) = &on_split_right { cb.call(()) },
                    on_split_down: move |_| if let Some(cb) = &on_split_down { cb.call(()) },
                    on_close_split: move |_| if let Some(cb) = &on_close_split { cb.call(()) },
                }

                div {
                    style: "flex: 1; display: flex; align-items: center; justify-content: center;",
                    span {
                        style: {
                            let colors = use_theme().colors();
                            format!("color: {};", colors.text_muted)
                        },
                        "No file selected"
                    }
                }
            }
        }
    }
}
