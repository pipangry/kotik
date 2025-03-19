use crate::utils::cipher::{generate_random_key, CipherError};
use crate::utils::cli::get_input;
use std::ffi::OsString;
use std::fs::{read_dir, File};
use std::io::{Error, Write};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::thread;

// Pack encryption is encrypt/decrypt commands

#[derive(Debug)]
pub enum PackEncryptionError {
    ContentsDecodingError(FromUtf8Error),
    ContentsGeneratingError(Error),
    JsonError(serde_json::Error),
    ProcessingError(Vec<String>),
    DataCollectionError(String),
    CipherError(CipherError),
    FileSystemError(Error),
    Abort,
}

// I'm manually creating and using write_all since for some
// reason when I use std::fs::write sometimes it can write
// zero bytes
pub fn write_file(bytes: &[u8], path: &PathBuf) -> Result<(), Error> {
    let mut contents_file = File::create(path)?;
    contents_file.write_all(bytes)?;
    Ok(())
}

// Function to collect relative paths of all directories and files
pub fn list_relative_paths(root: &OsString) -> std::io::Result<Vec<PathBuf>> {
    let root_path = Path::new(root);
    let mut entries = Vec::new();
    let mut dirs_to_visit = vec![PathBuf::new()];

    // Traverse directories
    while let Some(rel_path) = dirs_to_visit.pop() {
        let full_path = root_path.join(&rel_path);

        for entry in read_dir(full_path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let entry_rel_path = rel_path.join(&file_name);

            if entry.file_type()?.is_dir() {
                // TODO: try to refactor this without .clone()
                dirs_to_visit.push(entry_rel_path.clone());

                let mut modified_path = entry_rel_path.clone().into_os_string();

                // Buffer to hold the bytes of the separator
                let mut sep_buf = [0u8; 4];
                let sep_str = MAIN_SEPARATOR.encode_utf8(&mut sep_buf);
                /* For folders Minecraft needs '/' */
                modified_path.push(sep_str);
                entries.push(PathBuf::from(modified_path));
            } else {
                entries.push(entry_rel_path);
            }
        }
    }

    Ok(entries)
}

// Posted here to make code more readable
type ParallelProcessingResult = Result<(), Vec<String>>;

// Simple parallel processing for encrypt/decrypt tasks. Maybe you can make it better
pub fn parallel_processing<F, T>(tasks: Vec<T>, function: F) -> ParallelProcessingResult
where
    F: Fn(T) -> Result<(), String> + Send + Sync + 'static,
    T: Send + Sync + 'static + std::fmt::Debug,
{
    // For blazingly fast processing
    use crossbeam_channel::unbounded;

    let available_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let (sender, receiver) = unbounded();
    let function_arc = Arc::new(function);
    let mut handles = Vec::with_capacity(available_threads);

    // Producer thread
    let producer = thread::spawn(move || {
        for task in tasks {
            sender.send(task).unwrap_or_default();
        }
    });

    // Worker threads
    for _ in 0..available_threads {
        let receiver = receiver.clone();
        /* Sorry for another clone */
        let function = Arc::clone(&function_arc);
        handles.push(thread::spawn(move || {
            let mut errors = Vec::new();
            while let Ok(task) = receiver.recv() {
                if let Err(e) = function(task) {
                    errors.push(e);
                }
            }
            errors
        }));
    }

    let _ = producer.join();

    // Collecting errors
    let mut errors = Vec::new();
    for handle in handles {
        errors.extend(handle.join().unwrap());
    }

    // These errors will fall directly on your head, that is, all at once =)
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn parse_pack_encryption_args<F>(args: &[&str], command: F) -> Result<(), String>
where
    F: Fn(&str, OsString) -> Result<(), PackEncryptionError>,
{
    if args.is_empty() {
        return Err(String::from(
            "No arguments provided. Use 'help' to get list of all available commands.",
        ));
    }

    // For cases then user somehow forgot to specify path
    let path_arg = if args[1..].is_empty() {
        let mut input = String::new();
        get_input(
            "It looks like you forgot that you need to specify the folder path. Enter path:",
            &mut input,
        );
        input
    } else {
        // Sometimes paths can be with whitespaces, so we also handle that
        args[1..].join(" ")
    };

    let key = if args[0] == "-r" {
        &generate_random_key()
    } else {
        args[0]
    };
    let path = OsString::from(path_arg);

    (command)(key, path).map_err(|e| format!("Pack encryption error: {:#?}", e))?;

    println!("Key: {:?}", key);
    Ok(())
}
