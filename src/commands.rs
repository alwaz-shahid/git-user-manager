use crate::util::{display_divider, read_input, run_command};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

const USERS_FILE: &str = "users.json";

// pub fn list_users() {
//     let users = load_users();
//     println!("Current users:");
//     for user in users {
//         println!("{:?}", user);
//     }
// }
pub fn list_users() {
    let users = load_users();
    println!("All users list:");
    for (index, user) in users.iter().enumerate() {
        println!("{}: {:?}", index + 1, user); // Adding 1 to index for 1-based numbering
    }
}
pub fn current_user() {
    run_command("git config user.name && git config user.email");
}

pub fn add_user() {
    println!("ADD A NEW USER:");
    display_divider();

    let name = read_input("Enter your name: ");
    let email = read_input("Enter your email: ");
    let user = User { name, email };
    display_divider();

    println!(
        "Are you sure you want to add the following user?: {:?}",
        user
    );
    let choice = read_input("Confirm (y/n): ").to_lowercase();

    if choice.starts_with('y') {
        let mut users = load_users();
        users.push(user);
        save_users(&users);
        println!("User added successfully.");
        display_divider();
    } else {
        println!("User addition canceled.");
        display_divider();
    }
}

pub fn switch_user() {
    let users = load_users();

    if users.is_empty() {
        println!("No users available to switch.");
        return;
    }

    println!("All users list:");
    for (index, user) in users.iter().enumerate() {
        println!("{}: Name: {}, Email: {}", index + 1, user.name, user.email); // Adding 1 to index for 1-based numbering
    }

    println!("Enter the index of the user you want to switch to:");

    let input = read_input("Enter index: ");
    let index = input.trim().parse::<usize>();

    match index {
        Ok(idx) if idx > 0 && idx <= users.len() => {
            let selected_user = &users[idx - 1]; // Adjusting index to 0-based for vector access
            let command = format!(
                "git config --global user.name \"{}\" && git config --global user.email \"{}\"",
                selected_user.name, selected_user.email
            );

            // Execute the command
            run_command(&command);
            // print!("{} ", command);
            println!(
                "Switched to User: Name: {}, Email: {}",
                selected_user.name, selected_user.email
            );
            // Implement further actions with the selected user if needed
        }
        _ => {
            println!("Invalid index. No user switched.");
        }
    }
}

fn load_users() -> Vec<User> {
    let file = File::open(USERS_FILE).unwrap_or_else(|_| File::create(USERS_FILE).unwrap());
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

fn save_users(users: &[User]) {
    let file = File::create(USERS_FILE).expect("Unable to create file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &users).expect("Unable to write data");
}
