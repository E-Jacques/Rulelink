//! Generates (or checks) the committed JSON Schemas under `schemas/`.

use std::fs;
use std::path::Path;

use schemars::JsonSchema;
use schemars::generate::SchemaSettings;
use schemars::transform::RecursiveTransform;

pub fn run(check: bool) -> Result<(), String> {
    let dir = Path::new("schemas");
    fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let schemas = [
        // The Source is read: what may be left out follows deserialisation.
        (
            "rules.schema.json",
            schemars::schema_for!(rulelink_model::RuleSet),
        ),
        // Reports are written: what may be left out follows serialisation.
        (
            "check-report.schema.json",
            written::<rulelink_check::CheckReportV1>(),
        ),
    ];
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

/// The schema of what Rulelink writes as a `T`.
fn written<T: JsonSchema>() -> schemars::Schema {
    SchemaSettings::default()
        .for_serialize()
        // `schemars` names Rust integer types (`uint32`) as formats: not JSON Schema ones, and
        // strict validators refuse a format they do not know. `minimum` keeps the bound.
        .with_transform(RecursiveTransform(|schema: &mut schemars::Schema| {
            schema.remove("format");
        }))
        .into_generator()
        .into_root_schema_for::<T>()
}
