mod commands;
mod util;

use commands::{add_user, current_user, list_users, switch_user};
use util::{display_divider, read_input};

enum Cmds {
    ListUsers,
    CurrentUser,
    AddUser,
    SwitchUser,
    Invalid,
    Exit,
}

fn main() {
    let mut exit_program = false;
    display_divider();

    println!("Welcome to Git User Manager CLI Tool!\n");
    display_divider();
    println!("Version: 1.0.0");
    println!("Last Updated: 2024-07-18");
    println!("License: MIT");
    println!("Language: Rust");
    println!("Author: Alwaz Shahid");
    println!("GitHub: https://github.com/alwaz-shahid");
    println!("Description: Git User Manager is a command-line interface tool designed to simplify managing Git user configurations on your local machine. It provides an interactive menu for listing users, displaying the current user, adding a new user, and changing the active user seamlessly.");

    display_divider();

    while !exit_program {
        display_divider();

        println!("1- List users\n2- Current user\n3- Add user\n4- Switch user");
        display_divider();

        let input = read_input("Select an option: ");
        let cmd = match input.trim() {
            "1" => Cmds::ListUsers,
            "2" => Cmds::CurrentUser,
            "3" => Cmds::AddUser,
            "4" => Cmds::SwitchUser,
            "5" => Cmds::Exit,
            _ => Cmds::Invalid,
        };

        match cmd {
            Cmds::ListUsers => list_users(),
            Cmds::CurrentUser => current_user(),
            Cmds::AddUser => add_user(),
            Cmds::SwitchUser => switch_user(),
            Cmds::Invalid => invalid(),
            Cmds::Exit => exit_program = true,
        }

        display_divider();
        display_divider();
    }
}

fn invalid() {
    println!("Invalid command");
}
