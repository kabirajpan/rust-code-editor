use crate::theme::use_theme;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PanelType {
    Files,
    Search,
    Git,
    Settings,
}

#[component]
pub fn IconStrip(
    active_panel: Signal<Option<PanelType>>,
    on_panel_change: EventHandler<PanelType>,
) -> Element {
    rsx! {
        div {
            style: {
                let colors = use_theme().colors();
                format!("width: 30px; background-color: {}; display: flex; flex-direction: column; align-items: center;  border-right: 1px solid {};", colors.bg_secondary, colors.border_primary)
            },

            IconButton {
                panel_type: PanelType::Files,
                active_panel: active_panel,
                on_click: move |_| on_panel_change.call(PanelType::Files),
                svg {
                    view_box: "0 0 16 16",
                    width: "16",
                    height: "16",
                    fill: "currentColor",
                    path { d: "M14.5 2H7.71l-1.42-1.42A1 1 0 0 0 5.59 0H1.5A1.5 1.5 0 0 0 0 1.5v11A1.5 1.5 0 0 0 1.5 14h13a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2z" }
                }
            }

            IconButton {
                panel_type: PanelType::Search,
                active_panel: active_panel,
                on_click: move |_| on_panel_change.call(PanelType::Search),
                svg {
                    view_box: "0 0 16 16",
                    width: "16",
                    height: "16",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    circle { cx: "6.5", cy: "6.5", r: "5" }
                    line { x1: "10.5", y1: "10.5", x2: "15", y2: "15" }
                }
            }

            IconButton {
                panel_type: PanelType::Git,
                active_panel: active_panel,
                on_click: move |_| on_panel_change.call(PanelType::Git),
                svg {
                    view_box: "0 0 16 16",
                    width: "16",
                    height: "16",
                    fill: "currentColor",
                    circle { cx: "3", cy: "3", r: "2" }
                    circle { cx: "3", cy: "13", r: "2" }
                    circle { cx: "13", cy: "8", r: "2" }
                    path {
                        d: "M3 5 L3 11 M3 8 L11 8",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        fill: "none"
                    }
                }
            }

            IconButton {
                panel_type: PanelType::Settings,
                active_panel: active_panel,
                on_click: move |_| on_panel_change.call(PanelType::Settings),
                svg {
                    view_box: "0 0 16 16",
                    width: "16",
                    height: "16",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    circle { cx: "8", cy: "8", r: "3" }
                    line { x1: "8", y1: "0", x2: "8", y2: "3" }
                    line { x1: "8", y1: "13", x2: "8", y2: "16" }
                    line { x1: "0", y1: "8", x2: "3", y2: "8" }
                    line { x1: "13", y1: "8", x2: "16", y2: "8" }
                    line { x1: "2.5", y1: "2.5", x2: "4.5", y2: "4.5" }
                    line { x1: "11.5", y1: "11.5", x2: "13.5", y2: "13.5" }
                    line { x1: "2.5", y1: "13.5", x2: "4.5", y2: "11.5" }
                    line { x1: "11.5", y1: "4.5", x2: "13.5", y2: "2.5" }
                }
            }
        }
    }
}

#[component]
fn IconButton(
    panel_type: PanelType,
    active_panel: Signal<Option<PanelType>>,
    on_click: EventHandler<()>,
    children: Element,
) -> Element {
    let is_active = active_panel().map_or(false, |p| p == panel_type);
    let colors = use_theme().colors();
    let bg_color = if is_active { colors.bg_primary } else { "transparent" };
    let border = if is_active { format!("2px solid {}", colors.accent) } else { "2px solid transparent".to_string() };
    let color = if is_active { colors.text_primary } else { colors.text_muted };

    rsx! {
        div {
            style: "width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; cursor: pointer; background-color: {bg_color}; border-left: {border}; margin-bottom: 2px; color: {color};",
            onclick: move |_| on_click.call(()),
            {children}
        }
    }
}
