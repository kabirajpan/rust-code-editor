use super::types::{CursorPosition, EditorState};
use ropey::Rope;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct EditorAction {
    pub action_type: ActionType,
    pub position: usize,
    pub text: String,
    pub cursor_before: CursorPosition,
    pub cursor_after: CursorPosition,
}

#[derive(Clone, Debug)]
pub enum ActionType {
    Insert,
    Delete,
}

#[derive(Debug)]
pub struct RopeEditor {
    rope: Rope,
    file_path: PathBuf,
    is_modified: bool,
    cursor: CursorPosition,
    undo_stack: Vec<EditorAction>,
    redo_stack: Vec<EditorAction>,
    clipboard: String,
}

impl RopeEditor {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: PathBuf::new(),
            is_modified: false,
            cursor: CursorPosition {
                line: 0,
                column: 0,
                byte_offset: 0,
            },
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            clipboard: String::new(),
        }
    }

    pub fn load_file(&mut self, path: &PathBuf) -> Result<(), std::io::Error> {
        match std::fs::File::open(path) {
            Ok(file) => {
                self.rope = Rope::from_reader(file)?;
                self.file_path = path.clone();
                self.is_modified = false;
                self.cursor = CursorPosition {
                    line: 0,
                    column: 0,
                    byte_offset: 0,
                };
                self.undo_stack.clear();
                self.redo_stack.clear();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn save_file(&mut self) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(&self.file_path)?;
        self.rope.write_to(&mut file)?;
        self.is_modified = false;
        Ok(())
    }

    pub fn insert_text(&mut self, text: &str) {
        let position = self.cursor.byte_offset;
        if position <= self.rope.len_chars() && !text.is_empty() {
            let cursor_before = self.cursor.clone();

            self.rope.insert(position, text);
            self.is_modified = true;

            // Update cursor position after insertion
            self.cursor.byte_offset = position + text.chars().count();
            self.update_cursor_from_byte_offset();

            let cursor_after = self.cursor.clone();

            // Add to undo stack
            self.undo_stack.push(EditorAction {
                action_type: ActionType::Insert,
                position,
                text: text.to_string(),
                cursor_before,
                cursor_after,
            });

            // Clear redo stack when new action is performed
            self.redo_stack.clear();
        }
    }

    pub fn delete_range(&mut self, start: usize, end: usize) {
        if start < end && end <= self.rope.len_chars() {
            let cursor_before = self.cursor.clone();
            let deleted_text = self.rope.slice(start..end).to_string();

            self.rope.remove(start..end);
            self.is_modified = true;
            self.cursor.byte_offset = start;
            self.update_cursor_from_byte_offset();

            let cursor_after = self.cursor.clone();

            // Add to undo stack
            self.undo_stack.push(EditorAction {
                action_type: ActionType::Delete,
                position: start,
                text: deleted_text,
                cursor_before,
                cursor_after,
            });

            // Clear redo stack when new action is performed
            self.redo_stack.clear();
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor.byte_offset > 0 {
            let end = self.cursor.byte_offset;
            let start = end - 1;
            self.delete_range(start, end);
        }
    }

    pub fn delete(&mut self) {
        if self.cursor.byte_offset < self.rope.len_chars() {
            let start = self.cursor.byte_offset;
            let end = start + 1;
            self.delete_range(start, end);
        }
    }

    pub fn insert_newline(&mut self) {
        self.insert_text("\n");
    }

    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            match action.action_type {
                ActionType::Insert => {
                    // Reverse insertion by deleting
                    let start = action.position;
                    let end = start + action.text.chars().count();
                    if end <= self.rope.len_chars() {
                        self.rope.remove(start..end);
                    }
                }
                ActionType::Delete => {
                    // Reverse deletion by inserting
                    if action.position <= self.rope.len_chars() {
                        self.rope.insert(action.position, &action.text);
                    }
                }
            }

            self.cursor = action.cursor_before;
            self.redo_stack.push(action);
            self.is_modified = true;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(action) = self.redo_stack.pop() {
            match action.action_type {
                ActionType::Insert => {
                    // Redo insertion
                    if action.position <= self.rope.len_chars() {
                        self.rope.insert(action.position, &action.text);
                    }
                }
                ActionType::Delete => {
                    // Redo deletion
                    let start = action.position;
                    let end = start + action.text.chars().count();
                    if end <= self.rope.len_chars() {
                        self.rope.remove(start..end);
                    }
                }
            }

            self.cursor = action.cursor_after;
            self.undo_stack.push(action);
            self.is_modified = true;
            true
        } else {
            false
        }
    }

    pub fn copy_selection(&mut self, start: usize, end: usize) {
        if start < end && end <= self.rope.len_chars() {
            self.clipboard = self.rope.slice(start..end).to_string();
        }
    }

    pub fn copy_line(&mut self) {
        let line_start = self.rope.line_to_char(self.cursor.line);
        let line_end = if self.cursor.line + 1 < self.rope.len_lines() {
            self.rope.line_to_char(self.cursor.line + 1)
        } else {
            self.rope.len_chars()
        };
        self.clipboard = self.rope.slice(line_start..line_end).to_string();
    }

    pub fn paste(&mut self) {
        if !self.clipboard.is_empty() {
            self.insert_text(&self.clipboard.clone());
        }
    }

    pub fn get_line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.rope.len_lines() {
            let line = self.rope.line(line_idx);
            let line_str = line.to_string();
            Some(line_str.trim_end_matches(&['\n', '\r'][..]).to_string())
        } else {
            None
        }
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn total_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn get_content(&self) -> String {
        self.rope.to_string()
    }

    pub fn get_editor_state(&self) -> EditorState {
        EditorState {
            file_path: self.file_path.clone(),
            is_modified: self.is_modified,
            scroll_position: 0,
            cursor: self.cursor.clone(),
        }
    }

    pub fn set_cursor(&mut self, line: usize, column: usize) {
        if line < self.rope.len_lines() {
            let line_text = self.rope.line(line);
            let line_len = line_text.len_chars();
            // Allow cursor at end of line (after last character)
            let column = if line_len == 0 {
                0
            } else if line_text.to_string().ends_with('\n') {
                column.min(line_len.saturating_sub(1))
            } else {
                column.min(line_len)
            };

            let line_start = self.rope.line_to_char(line);
            self.cursor = CursorPosition {
                line,
                column,
                byte_offset: line_start + column,
            };
        }
    }

    pub fn get_cursor(&self) -> &CursorPosition {
        &self.cursor
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.line > 0 {
            let target_line = self.cursor.line - 1;
            self.set_cursor(target_line, self.cursor.column);
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.line + 1 < self.rope.len_lines() {
            let target_line = self.cursor.line + 1;
            self.set_cursor(target_line, self.cursor.column);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.column > 0 {
            self.set_cursor(self.cursor.line, self.cursor.column - 1);
        } else if self.cursor.line > 0 {
            let prev_line = self.cursor.line - 1;
            let prev_line_text = self.rope.line(prev_line);
            let prev_line_len = prev_line_text.len_chars();
            let target_col = if prev_line_text.to_string().ends_with('\n') {
                prev_line_len.saturating_sub(1)
            } else {
                prev_line_len
            };
            self.set_cursor(prev_line, target_col);
        }
    }

    pub fn move_cursor_right(&mut self) {
        let line_text = self.rope.line(self.cursor.line);
        let line_len = line_text.len_chars();
        let max_col = if line_text.to_string().ends_with('\n') {
            line_len.saturating_sub(1)
        } else {
            line_len
        };

        if self.cursor.column < max_col {
            self.set_cursor(self.cursor.line, self.cursor.column + 1);
        } else if self.cursor.line + 1 < self.rope.len_lines() {
            self.set_cursor(self.cursor.line + 1, 0);
        }
    }

    pub fn move_cursor_to_line_start(&mut self) {
        self.set_cursor(self.cursor.line, 0);
    }

    pub fn move_cursor_to_line_end(&mut self) {
        let line_text = self.rope.line(self.cursor.line);
        let line_len = line_text.len_chars();
        let target_col = if line_len == 0 {
            0
        } else if line_text.to_string().ends_with('\n') {
            line_len.saturating_sub(1)
        } else {
            line_len
        };
        self.set_cursor(self.cursor.line, target_col);
    }

    fn update_cursor_from_byte_offset(&mut self) {
        let byte_offset = self.cursor.byte_offset.min(self.rope.len_chars());
        let line = self.rope.char_to_line(byte_offset);
        let line_start = self.rope.line_to_char(line);
        let column = byte_offset - line_start;

        self.cursor.line = line;
        self.cursor.column = column;
        self.cursor.byte_offset = byte_offset;
    }
}
