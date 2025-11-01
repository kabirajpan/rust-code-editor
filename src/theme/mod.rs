use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    VSCode,
    Gruvbox,
    Atom,
    Monokai,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconTheme {
    VSCode,
    Material,
    Gruvbox,
    Atom,
}

#[derive(Clone, Debug)]
pub struct ThemeColors {
    // Background colors
    pub bg_primary: &'static str,
    pub bg_secondary: &'static str,
    pub bg_tertiary: &'static str,
    pub bg_accent: &'static str,

    // Text colors
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,

    // Border colors
    pub border_primary: &'static str,
    pub border_secondary: &'static str,

    // Accent colors
    pub accent: &'static str,
    pub accent_hover: &'static str,

    // Status colors
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,

    // Editor specific
    pub editor_bg: &'static str,
    pub editor_line_number: &'static str,
    pub editor_cursor: &'static str,
    pub editor_selection: &'static str,

    // Syntax highlighting
    pub syntax_keyword: &'static str,
    pub syntax_string: &'static str,
    pub syntax_comment: &'static str,
    pub syntax_number: &'static str,
    pub syntax_function: &'static str,
}

impl Theme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::VSCode => ThemeColors {
                bg_primary: "#1e1e1e",
                bg_secondary: "#2d2d30",
                bg_tertiary: "#252526",
                bg_accent: "#37373d",
                text_primary: "#cccccc",
                text_secondary: "#d4d4d4",
                text_muted: "#858585",
                border_primary: "#3e3e42",
                border_secondary: "#6e6e70",
                accent: "#007acc",
                accent_hover: "#1177bb",
                success: "#4caf50",
                warning: "#ff9800",
                error: "#f44747",
                editor_bg: "#1e1e1e",
                editor_line_number: "#858585",
                editor_cursor: "#aeafad",
                editor_selection: "#264f78",
                syntax_keyword: "#c586c0",
                syntax_string: "#ce9178",
                syntax_comment: "#6a9955",
                syntax_number: "#b5cea8",
                syntax_function: "#dcdcaa",
            },
            Theme::Gruvbox => ThemeColors {
                bg_primary: "#282828",
                bg_secondary: "#3c3836",
                bg_tertiary: "#32302f",
                bg_accent: "#504945",
                text_primary: "#ebdbb2",
                text_secondary: "#d5c4a1",
                text_muted: "#a89984",
                border_primary: "#665c54",
                border_secondary: "#7c6f64",
                accent: "#fe8019",
                accent_hover: "#d65d0e",
                success: "#b8bb26",
                warning: "#fabd2f",
                error: "#fb4934",
                editor_bg: "#282828",
                editor_line_number: "#a89984",
                editor_cursor: "#ebdbb2",
                editor_selection: "#458588",
                syntax_keyword: "#fb4934",
                syntax_string: "#b8bb26",
                syntax_comment: "#928374",
                syntax_number: "#d3869b",
                syntax_function: "#fabd2f",
            },
            Theme::Atom => ThemeColors {
                bg_primary: "#21252b",
                bg_secondary: "#2c313a",
                bg_tertiary: "#282c34",
                bg_accent: "#3a3f4b",
                text_primary: "#abb2bf",
                text_secondary: "#c8ccd4",
                text_muted: "#5c6370",
                border_primary: "#3e4452",
                border_secondary: "#4b5263",
                accent: "#568af2",
                accent_hover: "#4078d4",
                success: "#98c379",
                warning: "#e5c07b",
                error: "#e06c75",
                editor_bg: "#282c34",
                editor_line_number: "#636d83",
                editor_cursor: "#528bff",
                editor_selection: "#3e4451",
                syntax_keyword: "#c678dd",
                syntax_string: "#98c379",
                syntax_comment: "#5c6370",
                syntax_number: "#d19a66",
                syntax_function: "#61afef",
            },
            Theme::Monokai => ThemeColors {
                bg_primary: "#272822",
                bg_secondary: "#3e3d32",
                bg_tertiary: "#2f2f2a",
                bg_accent: "#49483e",
                text_primary: "#f8f8f2",
                text_secondary: "#f8f8f2",
                text_muted: "#75715e",
                border_primary: "#49483e",
                border_secondary: "#5e5d52",
                accent: "#66d9ef",
                accent_hover: "#4db8d9",
                success: "#a6e22e",
                warning: "#e6db74",
                error: "#f92672",
                editor_bg: "#272822",
                editor_line_number: "#90908a",
                editor_cursor: "#f8f8f0",
                editor_selection: "#49483e",
                syntax_keyword: "#f92672",
                syntax_string: "#e6db74",
                syntax_comment: "#75715e",
                syntax_number: "#ae81ff",
                syntax_function: "#a6e22e",
            },
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Theme::VSCode => "VS Code Dark",
            Theme::Gruvbox => "Gruvbox",
            Theme::Atom => "Atom One Dark",
            Theme::Monokai => "Monokai",
        }
    }
}

impl IconTheme {
    pub fn name(&self) -> &'static str {
        match self {
            IconTheme::VSCode => "VS Code Icons",
            IconTheme::Material => "Material Icons",
            IconTheme::Gruvbox => "Gruvbox Icons",
            IconTheme::Atom => "Atom Icons",
        }
    }
}

#[derive(Clone)]
pub struct ThemeContext {
    pub current_theme: Signal<Theme>,
    pub current_icon_theme: Signal<IconTheme>,
}

impl ThemeContext {
    pub fn new() -> Self {
        // Load saved values if available (web only); desktop falls back to defaults
        let (initial_theme, initial_icon_theme) = {
            #[cfg(target_arch = "wasm32")]
            {
                let window = web_sys::window();
                if let Some(win) = window {
                    if let Ok(Some(storage)) = win.local_storage() {
                        let theme = storage
                            .get_item("app.theme")
                            .ok()
                            .flatten()
                            .and_then(|name| match name.as_str() {
                                "VSCode" => Some(Theme::VSCode),
                                "Gruvbox" => Some(Theme::Gruvbox),
                                "Atom" => Some(Theme::Atom),
                                "Monokai" => Some(Theme::Monokai),
                                _ => None,
                            })
                            .unwrap_or(Theme::VSCode);

                        let icon_theme = storage
                            .get_item("app.icon_theme")
                            .ok()
                            .flatten()
                            .and_then(|name| match name.as_str() {
                                "VSCode" => Some(IconTheme::VSCode),
                                "Material" => Some(IconTheme::Material),
                                "Gruvbox" => Some(IconTheme::Gruvbox),
                                "Atom" => Some(IconTheme::Atom),
                                _ => None,
                            })
                            .unwrap_or(IconTheme::VSCode);

                        (theme, icon_theme)
                    } else {
                        (Theme::VSCode, IconTheme::VSCode)
                    }
                } else {
                    (Theme::VSCode, IconTheme::VSCode)
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                (Theme::VSCode, IconTheme::VSCode)
            }
        };

        let ctx = Self {
            current_theme: use_signal(|| initial_theme),
            current_icon_theme: use_signal(|| initial_icon_theme),
        };

        // Persist on change (web only)
        #[cfg(target_arch = "wasm32")]
        {
            let current_theme = ctx.current_theme.clone();
            let current_icon_theme = ctx.current_icon_theme.clone();
            use_effect(move || {
                let theme = current_theme();
                let icon = current_icon_theme();
                if let Some(win) = web_sys::window() {
                    if let Ok(Some(storage)) = win.local_storage() {
                        let _ = storage.set_item(
                            "app.theme",
                            match theme {
                                Theme::VSCode => "VSCode",
                                Theme::Gruvbox => "Gruvbox",
                                Theme::Atom => "Atom",
                                Theme::Monokai => "Monokai",
                            },
                        );
                        let _ = storage.set_item(
                            "app.icon_theme",
                            match icon {
                                IconTheme::VSCode => "VSCode",
                                IconTheme::Material => "Material",
                                IconTheme::Gruvbox => "Gruvbox",
                                IconTheme::Atom => "Atom",
                            },
                        );
                    }
                }
            });
        }

        ctx
    }

    pub fn colors(&self) -> ThemeColors {
        (self.current_theme)().colors()
    }
}

// Global theme context - initialize eagerly via the factory
static THEME_CONTEXT: GlobalSignal<ThemeContext> = Signal::global(|| ThemeContext::new());

pub fn use_theme() -> ThemeContext {
    THEME_CONTEXT().clone()
}

pub fn provide_theme_context() {
    // Already initialized by the global factory
}
