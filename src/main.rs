mod commands;
mod util;

use commands::{add_user, current_user, list_users, switch_user};
use dialoguer::{theme::ColorfulTheme, Select};
use util::display_divider;

fn main() {
    let mut exit_program = false;
    display_divider();

    println!("Welcome to Git User Manager CLI Tool!\n");
    display_divider();
    println!("Version: 1.0.1");
    println!("Last Updated: 2025-02-09");
    println!("License: MIT");
    println!("Language: Rust");
    println!("Author: Alwaz Shahid");
    println!("GitHub: https://github.com/alwaz-shahid");
    println!("Description: Git User Manager is a command-line interface tool designed to simplify managing Git user configurations on your local machine. It provides an interactive menu for listing users, displaying the current user, adding a new user, and changing the active user seamlessly.");

    display_divider();

    while !exit_program {
        let options = vec![
            "List users",
            "Current user",
            "Add user",
            "Switch user",
            "Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => list_users(),
            1 => current_user(),
            2 => add_user(),
            3 => switch_user(),
            4 => exit_program = true,
            _ => println!("Invalid selection. Please try again."),
        }

        display_divider();
    }
}