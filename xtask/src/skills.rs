//! Lints `skills/<name>/SKILL.md`: front matter must declare `name` and `description`.

use std::fs;
use std::path::Path;

pub fn run() -> Result<(), String> {
    let root = Path::new("skills");
    let mut errors = Vec::new();
    for entry in fs::read_dir(root).map_err(|e| e.to_string())?.flatten() {
        if !entry.path().is_dir() {
            continue;
        }
        let dir_name = entry.file_name().to_string_lossy().into_owned();
        let skill = entry.path().join("SKILL.md");
        let Ok(text) = fs::read_to_string(&skill) else {
            errors.push(format!("{dir_name}: missing SKILL.md"));
            continue;
        };
        if !text.starts_with("---\n") {
            errors.push(format!("{dir_name}: missing front matter"));
        }
        if !text.contains(&format!("name: {dir_name}\n")) {
            errors.push(format!(
                "{dir_name}: front matter `name` must equal the folder name"
            ));
        }
        if !text.contains("\ndescription:") {
            errors.push(format!("{dir_name}: front matter `description` missing"));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}
