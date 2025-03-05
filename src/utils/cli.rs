use std::io;
use std::io::{Write};
use std::process::exit;
use crate::packs::encryption::decrypt::decrypt;
use crate::packs::encryption::encrypt::encrypt;
use crate::packs::pack_encryption::{parse_pack_encryption_args};
use crate::utils::cipher::{generate_random_key};

#[derive(Debug)]
struct Command<F> {
    name: &'static str,
    description: &'static str,
    usage: &'static str,
    callback: F,
}

type Commands = [Command<fn(&[&str]) -> Result<(), String>>];

const COMMANDS: &Commands = &[
    Command {
        name: "help",
        description: "Get info about all available commands. To get information about a specific command, specify the name as the second argument",
        usage: "help <command(optional)>",
        callback: help
    },
    Command {
        name: "encrypt",
        description: "Encrypt a resource or behavior pack with given key and directory path",
        usage: "encrypt <key> <path>",
        callback: |args| {
            parse_pack_encryption_args(args, encrypt)
        }
    },
    Command {
        name: "decrypt",
        description: "Decrypt a resource or behavior pack with given key and directory path",
        usage: "decrypt < key> <path>",
        callback: |args| {
            parse_pack_encryption_args(args, decrypt)
        }
    },
    Command {
        name: "random_key",
        description: "Generate random 256 bit valid key for encryption",
        usage: "random_key",
        callback: |_| {
            println!("{}", generate_random_key());
            Ok(())
        }
    },
    Command {
        name: "exit",
        description: "Exit the program",
        usage: "exit",
        callback: |_| {
            exit(0);
        }
    },
    // Command {
    //     name: "test",
    //     description: "Test function",
    //     usage: "test",
    //     callback: test
    // }
];

// fn test(args: &[&str]) -> Result<(), String> {
//     Ok(())
// }

fn help(context: &[&str]) -> Result<(), String> {
    if context.is_empty() {
        println!("{}", COMMANDS.iter()
            .map(|cmd| format!("\"{}\" - {}", cmd.name, cmd.description))
            .collect::<Vec<_>>()
            .join("\n")
        );
        return Ok(());
    }

    let cmd = COMMANDS.iter()
        .find(|cmd| cmd.name == context[0])
        .ok_or(format!("Command '{}' not found", context[0]))?;

    println!("\"{}\" - {}\nUsage: \"{}\"\n", cmd.name, cmd.description, cmd.usage);
    Ok(())
}

pub fn handle_user_input(input: &str) -> Result<(), String> {
    let args: Vec<&str> = input.split_whitespace().collect();
    if let Some(cmd) = COMMANDS.iter().find(|cmd| cmd.name == args[0]) {
        (cmd.callback)(&args[1..])?;
        return Ok(())
    }
    Err(format!("Unknown command '{}'. Use 'help' to get full list of commands", args[0]))
}

pub fn get_input(prompt: &str, buffer: &mut String) {
    println!("{}", prompt);
    print!("> ");
    
    io::stdout().flush().expect("Failed to flush stdout");
    
    io::stdin().read_line(buffer)
        .unwrap_or_else(|e| {
            println!("Invalid input: {}", e);
            0
        });
}

pub fn get_choice(prompt: String) -> bool {
    let mut input = String::new();

    loop {
        println!("{} [y/n] ", prompt);
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => return true,
                    "n" | "no" => return false,
                    _ => {
                        println!("\nInvalid input. Please enter 'y' or 'n'.");
                    }
                }
            }
            Err(e) => {
                println!("\nError reading input: {}. Please try again.", e);
            }
        }
    }
}