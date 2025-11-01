use crate::layout::OpenFile;
use crate::theme::use_theme;
use dioxus::prelude::*;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let name = path.file_name()?.to_string_lossy().to_string();

        let is_dir = path.is_dir();
        let mut children = Vec::new();

        if is_dir {
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    if let Some(child) = FileNode::from_path(entry.path()) {
                        children.push(child);
                    }
                }
            }
            children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });
        }

        Some(FileNode {
            name,
            path,
            is_dir,
            children,
        })
    }
}

#[component]
pub fn FileTree(
    root_path: String,
    open_files: Signal<Vec<OpenFile>>,
    active_file_index: Signal<Option<usize>>,
) -> Element {
    let mut refresh_count = use_signal(|| 0);

    use_effect(move || {
        spawn(async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_secs(2)).await;
                refresh_count.set(refresh_count() + 1);
            }
        });
    });

    let file_tree = use_memo(move || {
        let _ = refresh_count();
        FileNode::from_path(PathBuf::from(root_path.clone()))
    });

    let selected_path = use_signal(|| None::<PathBuf>);

    rsx! {
        div {
            style: {
                let colors = use_theme().colors();
                format!("padding: 5px; color: {}; font-size: 0.85rem; user-select: none; overflow-x: hidden;", colors.text_primary)
            },
            if let Some(root) = file_tree.read().as_ref() {
                FileTreeNode {
                    node: root.clone(),
                    level: 0,
                    selected_path: selected_path,
                    open_files: open_files,
                    active_file_index: active_file_index
                }
            } else {
                div { "Failed to load directory" }
            }
        }
    }
}

#[component]
fn FileTreeNode(
    node: FileNode,
    level: i32,
    selected_path: Signal<Option<PathBuf>>,
    open_files: Signal<Vec<OpenFile>>,
    active_file_index: Signal<Option<usize>>,
) -> Element {
    let mut is_expanded = use_signal(|| level == 0);
    let indent = level * 12;

    let is_selected = selected_path().as_ref() == Some(&node.path);
    let colors = use_theme().colors();
    let bg_color = if is_selected {
        colors.bg_accent
    } else {
        "transparent"
    };

    let arrow_icon = if node.is_dir {
        if is_expanded() {
            "▾"
        } else {
            "▸"
        }
    } else {
        ""
    };

    let file_icon = {
        let theme = use_theme();
        let icon_theme = (theme.current_icon_theme)();
        if node.is_dir {
            match icon_theme {
                crate::theme::IconTheme::VSCode => "📁",
                crate::theme::IconTheme::Material => "🗂",
                crate::theme::IconTheme::Gruvbox => "🧰",
                crate::theme::IconTheme::Atom => "📂",
            }
        } else {
            let name = node.name.to_lowercase();
            let ext = std::path::Path::new(&name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            match (icon_theme, ext) {
                (_, "rs") => "🦀",
                (_, "js") => "🟨",
                (_, "ts") => "🟦",
                (_, "json") => "🧾",
                (_, "md") => "📝",
                (_, "toml") => "⚙️",
                (_, "yaml") | (_, "yml") => "📜",
                (_, "html") => "🌐",
                (_, "css") => "🎨",
                (_, "png") | (_, "jpg") | (_, "jpeg") | (_, "gif") | (_, "webp") => "🖼",
                _ => "📄",
            }
        }
    };

    let path_for_click = node.path.clone();
    let path_for_doubleclick = node.path.clone();
    let is_dir = node.is_dir;

    rsx! {
        div {
            style: "width: 100%;",

            div {
                style: "padding: 4px 8px; padding-left: {indent}px; cursor: pointer; user-select: none; display: flex; align-items: center; gap: 4px; background-color: {bg_color}; border-radius: 3px; white-space: nowrap; overflow: hidden;",

                onclick: move |evt| {
                    evt.stop_propagation();
                    selected_path.set(Some(path_for_click.clone()));

                    if is_dir {
                        is_expanded.set(!is_expanded());
                    } else {
                        // Check if file is already open
                        let files = open_files.read();
                        let existing_index = files.iter().position(|f| f.path == path_for_doubleclick);

                        if let Some(index) = existing_index {
                            // File already open, just switch to it
                            active_file_index.set(Some(index));
                        } else {
                            // Open new file
                            drop(files); // Release the read lock
                            let mut files = open_files.write();
                            files.push(OpenFile {
                                path: path_for_doubleclick.clone(),
                            });
                            let new_index = files.len() - 1;
                            active_file_index.set(Some(new_index));
                        }
                    }
                },

                if node.is_dir {
                    span {
                        style: "font-size: 0.8rem; color: #cccccc; width: 14px; display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0;",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            is_expanded.set(!is_expanded());
                        },
                        "{arrow_icon}"
                    }
                } else {
                    span {
                        style: "width: 14px; display: inline-block; flex-shrink: 0;"
                    }
                }

                span { style: "font-size: 0.85rem; flex-shrink: 0;", "{file_icon}" }

                span {
                    style: {
                        let colors = use_theme().colors();
                        format!("flex: 1; overflow: hidden; text-overflow: ellipsis; color: {};", colors.text_primary)
                    },
                    "{node.name}"
                }
            }

            if node.is_dir && is_expanded() {
                for child in &node.children {
                    FileTreeNode {
                        node: child.clone(),
                        level: level + 1,
                        selected_path: selected_path,
                        open_files: open_files,
                        active_file_index: active_file_index
                    }
                }
            }
        }
    }
}
