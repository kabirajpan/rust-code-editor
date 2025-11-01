use crate::layout::OpenFile;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Right,
    Down,
}

#[component]
pub fn TabBar(
    open_files: Signal<Vec<OpenFile>>,
    active_file_index: Signal<Option<usize>>,
    is_split: bool,
    on_split_right: EventHandler<()>,
    on_split_down: EventHandler<()>,
    on_close_split: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            style: "height: 30px; background-color: #252526; border-bottom: 1px solid #333; display: flex; align-items: center; justify-content: space-between; overflow-x: auto; overflow-y: visible; flex-shrink: 0; position: relative; z-index: 3000;",

            // Left side - tabs
            div {
                style: "display: flex; align-items: center; overflow-x: auto; flex: 1;",

                for (index, file) in open_files.read().iter().enumerate() {
                    {
                        let is_active = active_file_index() == Some(index);
                        let bg_color = if is_active { "#1e1e1e" } else { "#252526" };
                        let border_top = if is_active { "2px solid #007acc" } else { "2px solid transparent" };

                        let file_name = file.path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        rsx! {
                            div {
                                key: "{index}",
                                style: "background-color: {bg_color}; border-top: {border_top}; padding: 8px 12px; display: flex; align-items: center; gap: 8px; font-size: 0.85rem; color: #cccccc; cursor: pointer; border-right: 1px solid #333; user-select: none; flex-shrink: 0; min-width: 120px; max-width: 200px; white-space: nowrap;",
                                onclick: move |_| {
                                    active_file_index.set(Some(index));
                                },

                                span {
                                    style: "font-size: 0.85rem;",
                                    "📄"
                                }

                                span {
                                    style: "flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                    "{file_name}"
                                }

                                button {
                                    style: "background: none; border: none; color: #858585; cursor: pointer; padding: 0; width: 16px; height: 16px; display: flex; align-items: center; justify-content: center; font-size: 1.2rem; line-height: 1;",
                                    onclick: move |evt| {
                                        evt.stop_propagation();

                                        // Remove the file from the list
                                        let mut files = open_files.write();
                                        files.remove(index);

                                        // Update active index
                                        if files.is_empty() {
                                            active_file_index.set(None);
                                        } else if let Some(active_idx) = active_file_index() {
                                            if active_idx >= files.len() {
                                                active_file_index.set(Some(files.len() - 1));
                                            } else if active_idx == index && index > 0 {
                                                active_file_index.set(Some(index - 1));
                                            }
                                        }
                                    },
                                    "×"
                                }
                            }
                        }
                    }
                }
            }

            // Right side - single toggle icon [ | ]
            div {
                style: "display: flex; align-items: center; padding-right: 8px;",

                button {
                    style: "background: none; border: 1px solid #3e3e42; color: #cccccc; cursor: pointer; padding: 2px 8px; display: flex; align-items: center; justify-content: center; font-size: 0.9rem; border-radius: 4px; font-family: monospace;",
                    title: if is_split { "Close split" } else { "Split right" },
                    onclick: move |evt| {
                        evt.stop_propagation();
                        if is_split { on_close_split.call(()) } else { on_split_right.call(()) }
                    },
                    // Icon: [ | ]
                    span { "[ | ]" }
                }
            }
        }
    }
}