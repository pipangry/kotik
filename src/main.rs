use crate::utils::cli::{get_input, handle_user_input};

mod utils;
mod marketplace;
mod packs;

const BLUE: &str = "\x1B[0;34m";
const RED: &str = "\x1b[0;31m";
const RESET: &str = "\x1b[0m";

fn main() {
    println!("{}Welcome to Kotik - Pack encryption utility for Minecraft Bedrock{}\n", BLUE, RESET);
    
    loop {
        let mut input = String::new();
        get_input("Enter command or use 'help' for available commands:", &mut input);

        handle_user_input(input.trim())
            .unwrap_or_else(|e| println!("{}{}{}", RED, e, RESET));
    }
}