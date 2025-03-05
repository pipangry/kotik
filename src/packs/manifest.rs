use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::fs::metadata;
use serde_json::{Value};
use crate::utils::comments::clear_comments;

pub fn get_uuid_from_manifest(target_directory: &OsString) -> Result<String, String> {
    let mut path = PathBuf::from(target_directory);
    path.push("manifest.json");

    if let Ok(file_metadata) = metadata(&path) {
        if !file_metadata.is_file() {
            return Err(format!("Path is not a file: {}", path.display()));
        }
    } else {
        return Err(String::from("No manifest.json file in provided directory"));
    }

    let mut manifest_content = fs::read_to_string(&path)
        .map_err(|e| format!("Can't read manifest file: {}", e))?;

    clear_comments(&mut manifest_content);

    let json: Value = serde_json::from_str(&manifest_content)
        .map_err(|e| format!("Can't parse manifest file: {}", e))?;

    let uuid = json["header"]["uuid"]
        .as_str()
        .ok_or("Can't get uuid from manifest.json")?;

    Ok(uuid.to_owned())
}