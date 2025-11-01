use crate::theme::{use_theme, IconTheme, Theme};
use dioxus::desktop::use_window;
use dioxus::prelude::*;

#[component]
pub fn MenuBar(
    strip_visible: Signal<bool>,
    on_toggle_strip: EventHandler<()>,
    terminal_visible: Signal<bool>,
    on_toggle_terminal: EventHandler<()>,
    right_sidebar_visible: Signal<bool>,
    on_toggle_right_sidebar: EventHandler<()>,
    on_open_file: EventHandler<String>,
    on_open_folder: EventHandler<String>,
    on_new_file: EventHandler<()>,
    on_save_file: EventHandler<()>,
    on_save_as: EventHandler<()>,
) -> Element {
    let mut theme_dropdown_visible = use_signal(|| false);
    let mut icon_dropdown_visible = use_signal(|| false);
    let mut theme_context = use_theme();
    let colors = theme_context.colors();

    rsx! {
        div {
            style: "height: 30px; background-color: {colors.bg_secondary}; display: flex; align-items: center; justify-content: space-between;  border-bottom: 1px solid {colors.border_primary};",

            // Left side - controls and menu
            div {
                style: "display: flex; align-items: center; gap: 15px;",

                // Strip toggle button (NOT draggable)
                div {
                    style: "padding: 4px 9px; color: {colors.text_primary}; font-size: 0.85rem; cursor: pointer; background-color: {colors.bg_accent}; user-select: none;",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_toggle_strip.call(());
                    },
                    "☰"
                }

                // Non-draggable menu items
                div {
                    style: "display: flex; align-items: center; gap: 15px;",
                    MenuBarItem {
                        label: "File".to_string(),
                        on_open_file: on_open_file,
                        on_open_folder: on_open_folder,
                        on_new_file: on_new_file,
                        on_save_file: on_save_file,
                        on_save_as: on_save_as,
                    }
                    MenuBarItem {
                        label: "Edit".to_string(),
                        on_open_file: on_open_file,
                        on_open_folder: on_open_folder,
                        on_new_file: on_new_file,
                        on_save_file: on_save_file,
                        on_save_as: on_save_as,
                    }
                    MenuBarItem {
                        label: "View".to_string(),
                        on_open_file: on_open_file,
                        on_open_folder: on_open_folder,
                        on_new_file: on_new_file,
                        on_save_file: on_save_file,
                        on_save_as: on_save_as,
                    }
                    MenuBarItem {
                        label: "Selection".to_string(),
                        on_open_file: on_open_file,
                        on_open_folder: on_open_folder,
                        on_new_file: on_new_file,
                        on_save_file: on_save_file,
                        on_save_as: on_save_as,
                    }
                    MenuBarItem {
                        label: "Help".to_string(),
                        on_open_file: on_open_file,
                        on_open_folder: on_open_folder,
                        on_new_file: on_new_file,
                        on_save_file: on_save_file,
                        on_save_as: on_save_as,
                    }
                }

                // Add a small invisible draggable zone after menu
                div {
                    style: "width: 100px; height: 30px; cursor: move;",
                    onmousedown: move |_evt| {
                        let window = use_window();
                        window.drag();
                    }
                }
            }

            // Center - Window title (draggable area)
            div {
                style: "position: absolute; left: 50%; transform: translateX(-50%); color: {colors.text_primary}; font-size: 12px; font-weight: 500; cursor: move; user-select: none;",
                onmousedown: move |_evt| {
                    let window = use_window();
                    window.drag();
                },
                "Code Editor IDE"
            }

            // Right side - Toggle icons, status, and window controls
            div {
                style: "display: flex; align-items: center; gap: 8px;",

                // Toggle icons
                div {
                    style: "display: flex; align-items: center; gap: 4px; margin-right: 15px;",

                    // Left sidebar toggle
                    button {
                        style: "width: 24px; height: 24px; background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 12px;",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            on_toggle_strip.call(());
                        },
                        title: "Toggle Sidebar",
                        "⊞"
                    }

                    // Terminal toggle
                    button {
                        style: "width: 24px; height: 24px; background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 12px;",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            on_toggle_terminal.call(());
                        },
                        title: "Toggle Terminal",
                        "⌨"
                    }

                    // Right sidebar toggle
                    button {
                        style: "width: 24px; height: 24px; background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 12px;",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            on_toggle_right_sidebar.call(());
                        },
                        title: "Toggle Right Sidebar",
                        "⊟"
                    }
                }

                // Theme and icon dropdowns
                div {
                    style: "display: flex; align-items: center; gap: 8px; margin-right: 15px;",

                    // Theme dropdown
                    div {
                        style: "position: relative;",
                        button {
                            style: "background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; padding: 4px 8px; font-size: 11px; display: flex; align-items: center; gap: 4px;",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                theme_dropdown_visible.set(!theme_dropdown_visible());
                                icon_dropdown_visible.set(false);
                            },
                            "🎨 Theme"
                            span { style: "font-size: 8px;", "▼" }
                        }

                        if theme_dropdown_visible() {
                            div {
                                style: "position: absolute; top: 100%; right: 0; background-color: {colors.bg_secondary}; border: 1px solid {colors.border_primary}; border-radius: 4px; min-width: 120px; z-index: 1000; box-shadow: 0 4px 8px rgba(0,0,0,0.3);",
                                onclick: move |evt| evt.stop_propagation(),

                                for theme in [Theme::VSCode, Theme::Gruvbox, Theme::Atom, Theme::Monokai] {
                                    div {
                                        key: "{theme:?}",
                                        style: if (theme_context.current_theme)() == theme {
                                            format!("padding: 8px 12px; cursor: pointer; color: {}; font-size: 11px; background-color: {};", colors.text_primary, colors.accent)
                                        } else {
                                            format!("padding: 8px 12px; cursor: pointer; color: {}; font-size: 11px; hover: background-color: {};", colors.text_primary, colors.bg_accent)
                                        },
                                        onclick: move |_| {
                                            theme_context.current_theme.set(theme);
                                            theme_dropdown_visible.set(false);
                                        },
                                        if (theme_context.current_theme)() == theme { "✓ " } else { "" }
                                        "{theme.name()}"
                                    }
                                }
                            }
                        }
                    }

                    // Icon theme dropdown
                    div {
                        style: "position: relative;",
                        button {
                            style: "background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; padding: 4px 8px; font-size: 11px; display: flex; align-items: center; gap: 4px;",
                            onclick: move |evt| {
                                evt.stop_propagation();
                                icon_dropdown_visible.set(!icon_dropdown_visible());
                                theme_dropdown_visible.set(false);
                            },
                            "📁 Icons"
                            span { style: "font-size: 8px;", "▼" }
                        }

                        if icon_dropdown_visible() {
                            div {
                                style: "position: absolute; top: 100%; right: 0; background-color: {colors.bg_secondary}; border: 1px solid {colors.border_primary}; border-radius: 4px; min-width: 120px; z-index: 1000; box-shadow: 0 4px 8px rgba(0,0,0,0.3);",
                                onclick: move |evt| evt.stop_propagation(),

                                for icon_theme in [IconTheme::VSCode, IconTheme::Material, IconTheme::Gruvbox, IconTheme::Atom] {
                                    div {
                                        key: "{icon_theme:?}",
                                        style: if (theme_context.current_icon_theme)() == icon_theme {
                                            format!("padding: 8px 12px; cursor: pointer; color: {}; font-size: 11px; background-color: {};", colors.text_primary, colors.accent)
                                        } else {
                                            format!("padding: 8px 12px; cursor: pointer; color: {}; font-size: 11px;", colors.text_primary)
                                        },
                                        onclick: move |_| {
                                            theme_context.current_icon_theme.set(icon_theme);
                                            icon_dropdown_visible.set(false);
                                        },
                                        if (theme_context.current_icon_theme)() == icon_theme { "✓ " } else { "" }
                                        "{icon_theme.name()}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Status info
                div {
                    style: "display: flex; align-items: center; gap: 8px; color: {colors.text_muted}; font-size: 11px; margin-right: 15px;",
                    span { "⚡ Rust + Dioxus" }
                }

                // Window controls
                WindowControls {}
            }
        }
    }
}

// In menu_bar.rs, update the MenuBarItem component:

// In menu_bar.rs, update the MenuBarItem component to avoid capturing window in the outer closure:

#[component]
fn MenuBarItem(
    label: String,
    on_open_file: EventHandler<String>,
    on_open_folder: EventHandler<String>,
    on_new_file: EventHandler<()>,
    on_save_file: EventHandler<()>,
    on_save_as: EventHandler<()>,
) -> Element {
    let mut is_hovered = use_signal(|| false);
    let mut is_dropdown_open = use_signal(|| false);
    let colors = use_theme().colors();
    // Remove window from here and get it inside the specific closure where needed

    // Define dropdown items (for "File" only)
    let dropdown_items = match label.as_str() {
        "File" => Some(vec![
            "New File",
            "Open File",
            "Open Folder",
            "Save",
            "Save As...",
            "Exit",
        ]),
        _ => None,
    };

    rsx! {
        div {
            style: "position: relative;",

            // Top-level button
            div {
                style: format!(
                    "padding: 4px 12px; color: {}; font-size: 0.85rem; cursor: pointer; \
                     background-color: {}; user-select: none;",
                    colors.text_primary,
                    if is_hovered() { colors.bg_accent } else { "transparent" }
                ),
                onmouseenter: move |_| is_hovered.set(true),
                onmouseleave: move |_| is_hovered.set(false),
                onclick: move |evt| {
                    evt.stop_propagation();
                    if dropdown_items.is_some() {
                        is_dropdown_open.set(!is_dropdown_open());
                    }
                },
                "{label}"
            }

            // Dropdown menu (for File)
            if is_dropdown_open() {
                div {
                    style: format!(
                        "position: absolute; top: 100%; left: 0; background-color: {}; \
                         border: 1px solid {}; border-radius: 4px; min-width: 140px; \
                         z-index: 1000; box-shadow: 0 4px 8px rgba(0,0,0,0.3);",
                        colors.bg_secondary, colors.border_primary
                    ),
                    for item in dropdown_items.clone().unwrap_or_default() {
                        div {
                            key: "{item}",
                            style: format!(
                                "padding: 6px 12px; font-size: 0.8rem; color: {}; \
                                 cursor: pointer; user-select: none; \
                                 &:hover {{ background-color: {}; }}",
                                colors.text_primary,
                                colors.bg_accent
                            ),
                            onmouseenter: move |_| {},
                            onclick: move |evt| {
                                evt.stop_propagation();
                                is_dropdown_open.set(false);

                                // Handle each menu action
                                match item {
                                    "New File" => {
                                        on_new_file.call(());
                                    }
                                    "Open File" => {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            if let Some(path_str) = path.to_str() {
                                                on_open_file.call(path_str.to_string());
                                            }
                                        }
                                    }
                                    "Open Folder" => {
                                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                            if let Some(path_str) = path.to_str() {
                                                on_open_folder.call(path_str.to_string());
                                            }
                                        }
                                    }
                                    "Save" => {
                                        on_save_file.call(());
                                    }
                                    "Save As..." => {
                                        on_save_as.call(());
                                    }
                                    "Exit" => {
                                        // FIX: Get window inside this specific closure
                                        let window = use_window();
                                        window.close();
                                    }
                                    _ => {}
                                }
                            },
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn WindowControls() -> Element {
    let window = use_window();
    let window_clone1 = window.clone();
    let window_clone2 = window.clone();
    let window_clone3 = window.clone();
    let colors = use_theme().colors();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 1px;",

            // Minimize button
            button {
                style: "width: 30px; height: 22px; background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 16px; font-family: monospace;",
                onclick: move |evt| {
                    evt.stop_propagation();
                    window_clone1.set_minimized(true);
                },
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                "−"
            }

            // Maximize/Restore button
            button {
                style: "width: 30px; height: 22px; background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 14px; font-family: monospace;",
                onclick: move |evt| {
                    evt.stop_propagation();
                    let is_maximized = window_clone2.is_maximized();
                    window_clone2.set_maximized(!is_maximized);
                },
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                "□"
            }

            // Close button
            button {
                style: "width: 30px; height: 22px; background: transparent; border: none; color: {colors.text_primary}; cursor: pointer; display: flex; align-items: center; justify-content: center; font-size: 16px; font-family: monospace;",
                onclick: move |evt| {
                    evt.stop_propagation();
                    window_clone3.close();
                },
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                "×"
            }
        }
    }
}