use crate::packs::contents::ContentsRoot;
use crate::packs::pack_encryption::{parallel_processing, write_file, PackEncryptionError};
use crate::utils::cipher::aes256_cfb8_decrypt;
use crate::utils::cli::get_choice;
use std::ffi::OsString;
use std::fs::read;
use std::path::Path;

// This function can be represented as stages:
// 1. Parsing content.json
// 2. Decrypting all files
pub fn decrypt(key: &str, target_path: OsString) -> Result<(), PackEncryptionError> {
    // Asking user again
    if !get_choice(
        format!("Are you sure you want to decrypt the data on the following path: {:#?}? Files will be rewrote permanently.",
            target_path
        )
    ) {
        return Err(PackEncryptionError::Abort);
    }

    println!("Parsing contents.json file...");

    let contents_file_path = Path::new(&target_path).join("contents.json");
    let contents_file_content =
        read(&contents_file_path).map_err(PackEncryptionError::FileSystemError)?;

    let decrypted_content = aes256_cfb8_decrypt(
        key,
        // Removing header to get correct json
        contents_file_content[0x100..].to_vec(),
    )
    .map_err(PackEncryptionError::CipherError)?;

    // We will also write decrypted contents.json for more understanding
    write_file(&decrypted_content, &contents_file_path)
        .map_err(PackEncryptionError::FileSystemError)?;

    // Deserializing json
    let contents_root: ContentsRoot = serde_json::from_str(
        &String::from_utf8(decrypted_content)
            .map_err(PackEncryptionError::ContentsDecodingError)?,
    )
    .map_err(PackEncryptionError::JsonError)?;

    // Decrypting
    parallel_processing(contents_root.content, move |item| {
        let path = &item.path;
        let full_path = Path::new(&target_path).join(path);

        // We need this system call since if contents.json isn't
        // generated with Kotik, we can't verify that
        // all it paths is valid
        if full_path.is_dir() {
            return Ok(());
        }

        let key = match &item.key {
            Some(v) => v,
            None => return Ok(()),
        };

        let encrypted_file_content =
            read(&full_path).map_err(|e| format!("Can't read file: {}", e))?;

        let decrypted_file_content = aes256_cfb8_decrypt(key, encrypted_file_content)
            .map_err(|e| format!("Can't decrypt file: {:#?}", e))?;

        // Writing decrypted file
        write_file(&decrypted_file_content, &full_path)
            .map_err(|e| format!("Can't write file: {}", e))?;

        println!("Decrypted: {} with key {}", path, key);

        Ok(())
    })
    .map_err(PackEncryptionError::ProcessingError)?;
    Ok(())
}
