use super::rope_engine::RopeEditor;
use crate::theme::use_theme;
use dioxus::prelude::*;
use std::collections::HashMap;

// Cached line data to avoid repeated allocations
#[derive(Clone, Debug)]
struct CachedLine {
    content: String,
    char_count: usize,
    last_accessed: u64,
}

// Performance-optimized virtual editor with multiple improvements
#[component]
pub fn VirtualEditorView(editor: Signal<RopeEditor>, on_save: EventHandler<()>) -> Element {
    // Create a truly unique component ID for this specific editor instance
    let component_id = use_signal(|| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();

        // Hash the editor signal ID AND current time for true uniqueness
        let signal_id = format!("{:?}", editor.id());
        let time_val = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        signal_id.as_bytes().hash(&mut hasher);
        time_val.hash(&mut hasher);
        hasher.finish()
    });

    // Core state - isolated per buffer instance
    let mut first_visible_line = use_signal(|| 0usize);
    let mut is_focused = use_signal(|| false);
    let mut blink_visible = use_signal(|| true);
    let mut viewport_height = use_signal(|| 800.0);
    let mut frame_counter = use_signal(|| 0u64);

    // Performance optimizations - isolated per buffer
    let mut line_cache = use_signal(|| HashMap::<usize, CachedLine>::new());
    let _last_render_hash = use_signal(|| 0u64);
    let mut smooth_scroll_target = use_signal(|| None::<usize>);
    let mut scroll_momentum = use_signal(|| 0.0f64);

    // Constants - moved outside render loop with perfect alignment
    const LINE_HEIGHT: f64 = 20.0;
    const CHAR_WIDTH: f64 = 8.4;
    const LINE_NUMBERS_WIDTH: f64 = 50.0; // Increased for better spacing
    const BUFFER_SIZE: usize = 15; // Optimized buffer size
    const MAX_CACHE_SIZE: usize = 200; // Prevent memory bloat
    const SMOOTH_SCROLL_FACTOR: f64 = 0.85; // Smoothing factor

    // Optimized cursor blinking - isolated per buffer
    use_effect(move || {
        let comp_id = component_id();
        spawn(async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_millis(500)).await;
                if is_focused() {
                    blink_visible.set(!blink_visible());
                } else {
                    blink_visible.set(true);
                }
            }
        });
    });

    // Frame counter for performance tracking - isolated per buffer
    use_effect(move || {
        let comp_id = component_id();
        spawn(async move {
            loop {
                async_std::task::sleep(std::time::Duration::from_millis(16)).await;
                frame_counter.set(frame_counter() + 1);
            }
        });
    });

    // Smooth scrolling animation - isolated per buffer
    use_effect(move || {
        if let Some(target) = smooth_scroll_target() {
            spawn(async move {
                let current = first_visible_line();
                if current != target {
                    let diff = target as i32 - current as i32;
                    let step = (diff as f64 * 0.3).round() as i32;
                    if step.abs() >= 1 {
                        let new_pos = (current as i32 + step).max(0) as usize;
                        first_visible_line.set(new_pos);
                        async_std::task::sleep(std::time::Duration::from_millis(16)).await;
                    // 60fps
                    } else {
                        first_visible_line.set(target);
                        smooth_scroll_target.set(None);
                    }
                }
            });
        }
    });

    // Optimized virtual rendering with caching and memoization - isolated per buffer
    let (cursor_line, cursor_col, lines_data, cursor_top, cursor_left, _render_stats) = {
        let editor_read = editor.read();
        let line_count = editor_read.line_count();
        let first_line = first_visible_line();
        let current_frame = frame_counter();

        // Calculate viewport efficiently
        let lines_in_viewport = ((viewport_height() / LINE_HEIGHT) as f64).ceil() as usize;
        let start_line = first_line;
        let end_line = (first_line + lines_in_viewport + BUFFER_SIZE).min(line_count);

        // Create render hash for change detection - simplified
        let _render_hash = (first_line, end_line, editor_read.get_cursor().line);

        let cursor = editor_read.get_cursor();
        let cursor_line = cursor.line + 1; // Display line (1-based)
        let cursor_col = cursor.column + 1; // Display column (1-based)

        // Cursor positioning - optimized calculations
        let cursor_pixel_line = cursor.line;
        let cursor_pixel_col = cursor.column;
        let cursor_top = (cursor_pixel_line.saturating_sub(first_line)) as f64 * LINE_HEIGHT;

        // Optimized cursor left calculation with caching
        let cursor_left = {
            if cursor_pixel_line < line_count {
                let cache_key = cursor_pixel_line;
                let mut cache = line_cache.write();

                // Check cache first
                if let Some(cached) = cache.get(&cache_key) {
                    if cached.char_count > cursor_pixel_col {
                        LINE_NUMBERS_WIDTH + (cursor_pixel_col as f64 * CHAR_WIDTH)
                    } else {
                        LINE_NUMBERS_WIDTH + (cached.char_count as f64 * CHAR_WIDTH)
                    }
                } else if let Some(line_text) = editor_read.get_line(cursor_pixel_line) {
                    let char_count = line_text.chars().count();

                    // Cache the line data
                    cache.insert(
                        cache_key,
                        CachedLine {
                            content: line_text.clone(),
                            char_count,
                            last_accessed: current_frame,
                        },
                    );

                    let cursor_chars = cursor_pixel_col.min(char_count);
                    LINE_NUMBERS_WIDTH + (cursor_chars as f64 * CHAR_WIDTH)
                } else {
                    LINE_NUMBERS_WIDTH
                }
            } else {
                LINE_NUMBERS_WIDTH
            }
        };

        // Optimized line data collection with caching
        let mut lines_data = Vec::with_capacity(end_line - start_line);
        let mut cache = line_cache.write();

        // Clean old cache entries periodically
        if current_frame % 60 == 0 {
            cache.retain(|_, cached| current_frame - cached.last_accessed < 300);
        }

        for line_idx in start_line..end_line {
            let cache_key = line_idx;

            // Try cache first
            let line_content = if let Some(cached) = cache.get_mut(&cache_key) {
                cached.last_accessed = current_frame;
                cached.content.clone()
            } else if let Some(fresh_content) = editor_read.get_line(line_idx) {
                // Cache miss - fetch and cache
                if cache.len() < MAX_CACHE_SIZE {
                    cache.insert(
                        cache_key,
                        CachedLine {
                            content: fresh_content.clone(),
                            char_count: fresh_content.chars().count(),
                            last_accessed: current_frame,
                        },
                    );
                }
                fresh_content
            } else {
                continue;
            };

            let is_cursor_line = line_idx == cursor_pixel_line;
            let y_position = (line_idx.saturating_sub(first_line)) as f64 * LINE_HEIGHT;
            lines_data.push((line_idx, line_content, is_cursor_line, y_position));
        }

        drop(cache);
        drop(editor_read);

        // Performance stats for monitoring
        let render_stats = (
            lines_data.len(),
            line_count,
            current_frame,
            false, // Simplified to avoid signal writes in render
        );

        (
            cursor_line,
            cursor_col,
            lines_data,
            cursor_top,
            cursor_left,
            render_stats,
        )
    };

    // Optimized line rendering with reduced allocations
    let visible_lines_rsx =
        lines_data
            .into_iter()
            .map(|(line_idx, line_content, is_cursor_line, y_position)| {
                let theme_colors = use_theme().colors();
                let bg_color = if is_cursor_line { theme_colors.editor_selection } else { "transparent" };
                rsx! {
                    OptimizedLineComponent {
                        key: "{line_idx}",
                        line_idx: line_idx,
                        top_val: y_position,
                        line_content: line_content,
                        bg_color: bg_color,
                        is_cursor_line: is_cursor_line,
                    }
                }
            });

    rsx! {
        div {
            key: "editor_container_{component_id()}",
            style: "display: flex; flex-direction: column; height: 100vh; min-height: 100vh;",

            // Main editor area with optimizations and unique identity
            div {
                key: "editor_main_{component_id()}",
                style: {
                    let colors = use_theme().colors();
                    format!("flex: 1; background-color: {}; position: relative; outline: none; height: 100%; overflow: hidden; will-change: transform; contain: layout style paint; z-index: 1;", colors.editor_bg)
                },
                tabindex: 0,
                autofocus: true,

                onfocusin: move |_| {
                    is_focused.set(true);
                    blink_visible.set(true);
                },
                onfocusout: move |_| {
                    is_focused.set(false);
                    blink_visible.set(false);
                },

                onclick: move |_| {
                    is_focused.set(true);
                    blink_visible.set(true);
                },

                // Optimized mouse wheel with momentum and smoothing
                onwheel: move |evt| {
                    let delta_y = evt.delta().strip_units().y;
                    let momentum = scroll_momentum();

                    // Apply momentum and smoothing
                    let effective_delta = delta_y + momentum * 0.1;
                    let lines_to_scroll = (effective_delta / LINE_HEIGHT * 2.5) as i32;

                    if lines_to_scroll != 0 {
                        let editor_read = editor.read();
                        let line_count = editor_read.line_count();
                        let lines_in_viewport = ((viewport_height() / LINE_HEIGHT) as f64).ceil() as usize;
                        let max_first_line = line_count.saturating_sub(lines_in_viewport);
                        drop(editor_read);

                        let current_first = first_visible_line() as i32;
                        let new_first = (current_first + lines_to_scroll)
                            .max(0)
                            .min(max_first_line as i32) as usize;

                        // Update momentum for smooth scrolling
                        scroll_momentum.set(effective_delta * 0.8);

                        // Direct update for responsiveness, smooth scroll for large jumps
                        if lines_to_scroll.abs() > 5 {
                            smooth_scroll_target.set(Some(new_first));
                        } else {
                            first_visible_line.set(new_first);
                        }
                    }
                    evt.prevent_default();
                },

                // Optimized keyboard handling
                onkeypress: move |evt| {
                    if !evt.modifiers().ctrl() && !evt.modifiers().alt() {
                        if let Key::Character(ref s) = evt.key() {
                            if s.chars().all(|c| c.is_ascii_graphic() || c == ' ') {
                                let mut editor_write = editor.write();
                                editor_write.insert_text(s);
                                evt.prevent_default();

                                // Clear cache on modification
                                line_cache.write().clear();
                            }
                        }
                    }
                },

                onkeydown: move |evt| {
                    let key = evt.key();
                    let ctrl = evt.modifiers().ctrl();
                    let shift = evt.modifiers().shift();

                    match (ctrl, shift, key) {
                        // File operations
                        (true, false, Key::Character(ref s)) if s == "s" => {
                            evt.prevent_default();
                            on_save.call(());
                        }
                        // Undo/Redo
                        (true, false, Key::Character(ref s)) if s == "z" => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.undo();
                            line_cache.write().clear();
                        }
                        (true, true, Key::Character(ref s)) if s == "z" => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.redo();
                            line_cache.write().clear();
                        }
                        (true, false, Key::Character(ref s)) if s == "y" => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.redo();
                            line_cache.write().clear();
                        }
                        // Copy/Paste
                        (true, false, Key::Character(ref s)) if s == "c" => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.copy_line();
                        }
                        (true, false, Key::Character(ref s)) if s == "v" => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.paste();
                            line_cache.write().clear();
                        }

                        // Optimized cursor navigation with smart scrolling
                        (false, false, Key::ArrowUp) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.move_cursor_up();
                            let cursor_line = editor_write.get_cursor().line;
                            drop(editor_write);

                            // Smart auto-scroll with look-ahead
                            if cursor_line < first_visible_line() {
                                let new_first = cursor_line.saturating_sub(5); // Keep cursor away from edge
                                smooth_scroll_target.set(Some(new_first));
                            }
                        }
                        (false, false, Key::ArrowDown) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.move_cursor_down();
                            let cursor_line = editor_write.get_cursor().line;
                            drop(editor_write);

                            let lines_in_viewport = ((viewport_height() / LINE_HEIGHT) as f64).ceil() as usize;
                            if cursor_line >= first_visible_line() + lines_in_viewport.saturating_sub(5) {
                                let new_first = cursor_line.saturating_sub(lines_in_viewport.saturating_sub(10));
                                smooth_scroll_target.set(Some(new_first));
                            }
                        }
                        (false, false, Key::ArrowLeft) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.move_cursor_left();
                        }
                        (false, false, Key::ArrowRight) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.move_cursor_right();
                        }

                        // Home/End
                        (false, false, Key::Home) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.move_cursor_to_line_start();
                        }
                        (false, false, Key::End) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.move_cursor_to_line_end();
                        }

                        // Fast page navigation
                        (false, false, Key::PageUp) => {
                            evt.prevent_default();
                            let lines_in_viewport = ((viewport_height() / LINE_HEIGHT) as f64).ceil() as usize;
                            let new_first = first_visible_line().saturating_sub(lines_in_viewport);
                            smooth_scroll_target.set(Some(new_first));
                        }
                        (false, false, Key::PageDown) => {
                            evt.prevent_default();
                            let editor_read = editor.read();
                            let line_count = editor_read.line_count();
                            drop(editor_read);

                            let lines_in_viewport = ((viewport_height() / LINE_HEIGHT) as f64).ceil() as usize;
                            let max_first_line = line_count.saturating_sub(lines_in_viewport);
                            let new_first = (first_visible_line() + lines_in_viewport).min(max_first_line);
                            smooth_scroll_target.set(Some(new_first));
                        }

                        // Text editing
                        (false, false, Key::Backspace) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.backspace();
                            line_cache.write().clear();
                        }
                        (false, false, Key::Delete) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.delete();
                            line_cache.write().clear();
                        }
                        (false, false, Key::Enter) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.insert_newline();
                            line_cache.write().clear();
                        }
                        (false, false, Key::Tab) => {
                            evt.prevent_default();
                            let mut editor_write = editor.write();
                            editor_write.insert_text("    ");
                            line_cache.write().clear();
                        }
                        _ => {}
                    }
                },

                onmounted: move |evt| {
                    if let Some(element) = evt.data.downcast::<web_sys::Element>() {
                        // Get the full available height, accounting for parent container
                        let client_height = element.client_height() as f64;
                        let parent_height = if let Some(parent) = element.parent_element() {
                            parent.client_height() as f64
                        } else {
                            client_height
                        };

                        // Use the larger of the two, minus space for status bar and breadcrumbs
                        let effective_height = parent_height.max(client_height) - 70.0;  // Account for UI elements
                        viewport_height.set(effective_height.max(400.0));
                    }
                },

                onresize: move |_| {
                    // Re-calculate viewport height on resize
                    spawn(async move {
                        async_std::task::sleep(std::time::Duration::from_millis(100)).await;
                        // Force a viewport recalculation
                        let current = viewport_height();
                        viewport_height.set(current + 1.0);
                        viewport_height.set(current);
                    });
                },

                // Optimized virtual viewport with proper height calculation and unique identity
                div {
                    key: "viewport_{component_id()}",
                    style: "position: relative; width: 100%; height: calc(100% - 30px); overflow: hidden; will-change: transform; transform: translateZ(0);",
                    {visible_lines_rsx}

                    // Optimized cursor with GPU acceleration and unique identity
                    if is_focused() && blink_visible() && cursor_top >= 0.0 {
                        div {
                            key: "cursor_{component_id()}",
                            style: {
                                let colors = use_theme().colors();
                                format!("position: absolute; top: {cursor_top}px; left: {cursor_left}px; width: 1px; height: {LINE_HEIGHT}px; background-color: {}; z-index: 1000; pointer-events: none; will-change: transform; transform: translateZ(0);", colors.editor_cursor)
                            },
                        }
                    }
                }
            }

            // Optimized status bar with unique identity
            div {
                key: "status_{component_id()}",
                style: {
                    let colors = use_theme().colors();
                    format!("height: 24px; background-color: {}; color: {}; display: flex; align-items: center; padding: 0 15px; font-size: 0.75rem; font-family: 'Consolas', monospace; justify-content: space-between; flex-shrink: 0; border-top: 1px solid {};", colors.accent, colors.bg_primary, colors.border_primary)
                },
                span {
                    style: "font-weight: 500;",
                    "Ln {cursor_line}, Col {cursor_col}"
                }
                span {
                    style: "font-size: 0.7rem; opacity: 0.9;",
                    "Ctrl+S: Save • PgUp/PgDn: Scroll • Smooth Virtual Scrolling Active"
                }
            }
        }
    }
}

// Optimized line component with reduced re-renders
#[component]
fn OptimizedLineComponent(
    line_idx: usize,
    top_val: f64,
    line_content: String,
    bg_color: &'static str,
    is_cursor_line: bool,
) -> Element {
    const LINE_HEIGHT: f64 = 20.0;

    rsx! {
        div {
            style: "position: absolute; top: {top_val}px; left: 0; right: 0; height: {LINE_HEIGHT}px; display: flex; align-items: center; padding: 0 8px; font-family: 'Consolas', monospace; font-size: 14px; color: #d4d4d4; white-space: pre; background-color: {bg_color}; will-change: transform; transform: translateZ(0); contain: layout style paint;",

            // Line number
            span {
                style: {
                    let colors = use_theme().colors();
                    format!("color: {}; margin-right: 14px; width: 32px; text-align: right; font-size: 14px; user-select: none; flex-shrink: 0; font-weight: 400; font-family: 'Consolas', 'Monaco', 'Courier New', monospace; line-height: {LINE_HEIGHT}px; display: flex; align-items: center; justify-content: flex-end;", colors.editor_line_number)
                },
                "{line_idx + 1}"
            }

            // Line content with optimized rendering
            // Render tokens with basic syntax highlighting
            {
                let colors = use_theme().colors();
                let tokens = tokenize_line(&line_content);
                rsx! {
                    span {
                        style: "font-family: 'Consolas', 'Monaco', 'Courier New', monospace; white-space: pre; user-select: text; letter-spacing: 0; font-size: 14px; line-height: {LINE_HEIGHT}px; contain: layout style; flex: 1; display: flex; align-items: center;",
                        for (text, class_) in tokens {
                            span {
                                style: match class_ {
                                    TokenClass::Keyword => format!("color: {};", colors.syntax_keyword),
                                    TokenClass::String => format!("color: {};", colors.syntax_string),
                                    TokenClass::Comment => format!("color: {}; font-style: italic;", colors.syntax_comment),
                                    TokenClass::Number => format!("color: {};", colors.syntax_number),
                                    TokenClass::Function => format!("color: {};", colors.syntax_function),
                                    TokenClass::Plain => format!("color: {};", colors.text_primary),
                                },
                                "{text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum TokenClass { Keyword, String, Comment, Number, Function, Plain }

fn tokenize_line(line: &str) -> Vec<(String, TokenClass)> {
    // Very simple, non-stateful tokenizer for common patterns
    let mut out = Vec::new();
    let mut chars = line.chars().peekable();

    // Helpers
    let is_ident_start = |c: char| c.is_ascii_alphabetic() || c == '_' || c == '$';
    let is_ident_part = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '$';

    // Keyword sets combined for Rust/JS/TS/General
    let keywords = [
        // Rust
        "fn","let","mut","struct","enum","impl","trait","pub","use","mod","match","if","else","while","for","in","loop","return","break","continue","const","static","crate","super","self","Self","as","where","type","move","ref","async","await","dyn","unsafe",
        // JS/TS
        "function","var","const","let","class","interface","extends","implements","import","from","export","return","if","else","for","while","do","switch","case","break","continue","new","this","super","try","catch","finally","throw","await","async","yield","typeof","instanceof","in","of","void","delete",
        // General
        "true","false","null","undefined",
    ];

    while let Some(&c) = chars.peek() {
        // Comments
        if c == '/' {
            let mut it = chars.clone();
            it.next();
            if let Some('/') = it.next() {
                // // comment
                let mut text = String::new();
                while let Some(ch) = chars.next() { text.push(ch); }
                out.push((text, TokenClass::Comment));
                break;
            }
        }
        if c == '#' { // shell/py comment style
            let mut text = String::new();
            while let Some(ch) = chars.next() { text.push(ch); }
            out.push((text, TokenClass::Comment));
            break;
        }

        // Strings
        if c == '"' || c == '\'' || c == '`' {
            let quote = c;
            let mut text = String::new();
            text.push(chars.next().unwrap());
            let mut escaped = false;
            while let Some(ch) = chars.next() {
                text.push(ch);
                if escaped { escaped = false; continue; }
                if ch == '\\' { escaped = true; continue; }
                if ch == quote { break; }
            }
            out.push((text, TokenClass::String));
            continue;
        }

        // Numbers
        if c.is_ascii_digit() {
            let mut text = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_hexdigit() || ch == 'x' || ch == 'b' || ch == 'o' || ch == '_' || ch == '.' { text.push(ch); chars.next(); } else { break; }
            }
            out.push((text, TokenClass::Number));
            continue;
        }

        // Identifiers / keywords / functions
        if is_ident_start(c) {
            let mut ident = String::new();
            ident.push(chars.next().unwrap());
            while let Some(&ch) = chars.peek() { if is_ident_part(ch) { ident.push(ch); chars.next(); } else { break; } }

            // Function heuristic: followed by '(' with no space (or with spaces)
            let mut look = chars.clone();
            let mut saw_ws = false;
            while let Some(&ch) = look.peek() { if ch.is_whitespace() { saw_ws = true; look.next(); } else { break; } }
            let is_func = matches!(look.peek(), Some('('));

            if keywords.contains(&ident.as_str()) {
                out.push((ident, TokenClass::Keyword));
            } else if is_func {
                out.push((ident, TokenClass::Function));
            } else {
                out.push((ident, TokenClass::Plain));
            }
            continue;
        }

        // Single char fallback
        out.push((chars.next().unwrap().to_string(), TokenClass::Plain));
    }

    out
}