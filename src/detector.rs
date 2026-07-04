use std::fs;

pub fn detect(path: &str) -> Option<&'static str> {
    // Detect language from file path or extension.
    if let Some(lang) = tree_sitter_language_pack::detect_language(path) {
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
