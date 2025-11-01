use std::path::PathBuf;

#[derive(Clone, Debug, Copy)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

#[derive(Clone, Debug)]
pub struct EditorState {
    pub file_path: PathBuf,
    pub is_modified: bool,
    pub scroll_position: usize,
    pub cursor: CursorPosition,
}
