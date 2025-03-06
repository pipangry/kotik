use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use serde::{Deserialize, Serialize};
use crate::utils::cipher::generate_random_key;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentsRootItem {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentsRoot {
    pub version: i32,
    pub content: Vec<ContentsRootItem>,
}

// Generating contents.json header
pub fn generate_contents_header(uuid: &str) -> std::io::Result<Vec<u8>> {
    // Header is always 256 bytes
    let mut buffer = Vec::with_capacity(0x100);

    // 16 bytes
    buffer.write_all(&0i32.to_le_bytes())?;

    // NOTICE: for some reason marketplace packs have different magic number
    // Marketplace bytes: [239, 252, 191, 189, 239, 191, 189, 207]
    // You can get this magic number without decryption
    buffer.write_all(&0x9BCFB9FCu32.to_le_bytes())?;
    buffer.write_all(&0i64.to_le_bytes())?;

    let uuid_bytes = uuid.as_bytes();
    if uuid_bytes.len() > 255 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "UUID exceeds maximum length of 255 bytes"
        ));
    }

    // UUID section (240 bytes)
    buffer.write_all(&[uuid_bytes.len() as u8])?;
    buffer.write_all(uuid_bytes)?;
    let padding = 0xEF - uuid_bytes.len();

    // Pad to 239 bytes (0xEF)
    buffer.write_all(&vec![0u8; padding][..])?;

    Ok(buffer)
}

pub fn generate_contents_root(relative_paths: &[PathBuf]) -> Vec<ContentsRootItem> {
    relative_paths.iter()
        .map(|rel_path| ContentsRootItem {
            path: rel_path.to_string_lossy().replace(MAIN_SEPARATOR, "/"),
            key: if should_generate_key(rel_path) {
                Some(generate_random_key())
            } else {
                None
            }
        })
        .collect::<Vec<ContentsRootItem>>()
}

pub const DONT_ENCRYPT: [&str; 4] = [
    "manifest.json",
    "contents.json",
    "pack_icon.png",
    "texts/",
];

// Check if it folder or some of DO_NOT_ENCRYPT files
fn should_generate_key(path: &Path) -> bool {
    // string_lossy is cheap conversion, but you always
    // can debunk my code
    let path_as_string = path.to_string_lossy();
    // Key don't needed for folders
    if path_as_string.ends_with(MAIN_SEPARATOR) {
        return false;
    }

    !DONT_ENCRYPT.iter().any(|pattern| {
        path_as_string.starts_with(pattern)
    })
}