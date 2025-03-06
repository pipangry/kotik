use std::ffi::OsString;
use std::fs::{read};
use std::io::Write;
use std::path::{Path};
use serde_json::json;
use crate::packs::contents::{generate_contents_header, generate_contents_root, ContentsRoot};
use crate::packs::manifest::get_uuid_from_manifest;
use crate::packs::pack_encryption::{list_relative_paths, parallel_processing, write_file, PackEncryptionError};
use crate::utils::cipher::{aes256_cbf8_encrypt};
use crate::utils::cli::get_choice;

// This function can be represented as stages:
// 1. Collecting uuid and relative paths
// 2. Generating and writing contents.json file
// 3. Encrypting files
pub fn encrypt(key: &str, target_path: OsString) -> Result<(), PackEncryptionError> {
    println!("Collecting data...");
    
    let uuid = get_uuid_from_manifest(&target_path)
        .map_err(PackEncryptionError::DataCollectionError)?;

    let relative_paths = list_relative_paths(&target_path)
        .map_err(PackEncryptionError::FileSystemError)?;

    // Ask user once again
    if !get_choice(
        format!("Are you sure you want to encrypt the data on the following path: {:#?}? Files will be rewrote permanently.",
                target_path)
    ) {
        return Err(PackEncryptionError::Abort);
    }

    // Start with generating contents.json file
    let mut content_file_as_bytes = generate_contents_header(&uuid)
        .map_err(PackEncryptionError::ContentsGeneratingError)?;

    let content = generate_contents_root(&relative_paths);

    let root = ContentsRoot {
        version: 1,
        content,
    };

    let root_as_json_in_bytes = json!(root).to_string().as_bytes().to_vec();

    // Encrypting contents.json file root and writing it to the target path
    let encrypted_root = aes256_cbf8_encrypt(key, root_as_json_in_bytes)
        .map_err(PackEncryptionError::CipherError)?;

    content_file_as_bytes.write_all(&encrypted_root)
        .map_err(PackEncryptionError::ContentsGeneratingError)?;

    let contents_file_path = Path::new(&target_path).join("contents.json");
    
    write_file(&content_file_as_bytes, &contents_file_path)
        .map_err(PackEncryptionError::FileSystemError)?;

    // Encrypting files
    parallel_processing(root.content, move |item| {
        let path = &item.path;
        let full_path = Path::new(&target_path).join(path);
        
        let key = match &item.key {
            Some(key) => key,
            None => { return Ok(()) }
        };

        // We need to read it after key validation since folders don't have
        // key, and we can escape calling system check
        let file_content = read(&full_path)
            .map_err(|e| format!("Can't read file {}: {}", &path, e))?;
        
        let encrypted_file_content = aes256_cbf8_encrypt(key, file_content)
            .map_err(|e| format!("Can't encrypt: {:#?}", e))?;
        
        // let encrypted_file_content_as_bytes_list = encrypted_file_content.iter()
        //     .map(|b| b.to_string())
        //     .collect::<Vec<String>>()
        //     .join("\n");

        write_file(&encrypted_file_content, &full_path)
            .map_err(|e| format!("Can't write file: {}", e))?;
        
        println!("Encrypted: {}", path);
        Ok(())
    }).map_err(PackEncryptionError::ProcessingError)?;
    Ok(())
}