//! Generates (or checks) the committed JSON Schemas under `schemas/`.

use std::fs;
use std::path::Path;

pub fn run(check: bool) -> Result<(), String> {
    let dir = Path::new("schemas");
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let schemas = [(
        "rule.schema.json",
        schemars::schema_for!(checkmate_model::Rule),
    )];
    for (file, schema) in schemas {
        let mut body = serde_json::to_string_pretty(&schema).map_err(|e| e.to_string())?;
        body.push('\n');
        let path = dir.join(file);
        if check {
            let current = fs::read_to_string(&path).unwrap_or_default();
            if current != body {
                return Err(format!(
                    "schema drift: {} is out of date, run `cargo xtask schemas`",
                    path.display()
                ));
            }
        } else {
            fs::write(&path, body).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
