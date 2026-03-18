use std::path::PathBuf;

pub struct Location {
    pub file_path: PathBuf,
    pub line_number: Option<usize>,
    pub column_number: Option<usize>,
    pub snippet: Option<String>,
}

pub struct Diagnostic {
    pub title: String,
    pub help_text: String,
    pub location: Option<Location>,
}
