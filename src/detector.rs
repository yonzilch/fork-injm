use std::{fs, path::Path};

pub fn detect(path: &Path) -> Option<&'static str> {
    // Detect language from file path or extension.
    if let Some(lang) = tree_sitter_language_pack::detect_language(path.to_str()?) {
        return Some(lang);
    }

    // Detect from shebang line.
    if let Ok(content) = fs::read_to_string(path) {
        if let Some(lang) = tree_sitter_language_pack::detect_language_from_content(&content) {
            return Some(lang);
        }
    }

    None
}
