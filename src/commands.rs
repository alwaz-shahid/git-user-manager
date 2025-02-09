use crate::util::{display_divider, run_command};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

const USERS_FILE: &str = "git_users.json";

fn get_users_file_path() -> PathBuf {
    if let Some(home) = home_dir() {
        home.join(USERS_FILE)
    } else {
        PathBuf::from(USERS_FILE)
    }
}

pub fn list_users() {
    let users = load_users().unwrap_or_else(|err| {
        eprintln!("Error loading users: {}", err);
        Vec::new()
    });

    if users.is_empty() {
        println!("No users available.");
        return;
    }

    println!("All users list:");
    for (index, user) in users.iter().enumerate() {
        println!("{}: Name: {}, Email: {}", index + 1, user.name, user.email);
    }
}

pub fn current_user() {
    run_command("git config user.name && git config user.email");
}

pub fn add_user() {
    println!("ADD A NEW USER:");
    display_divider();

    let name = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your name")
        .interact_text()
        .unwrap();

    let email = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your email")
        .interact_text()
        .unwrap();

    let user = User { name, email };
    display_divider();

    let confirm = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Are you sure you want to add the following user?\n{:?}",
            user
        ))
        .interact()
        .unwrap();

    if confirm {
        let mut users = load_users().unwrap_or_else(|_| Vec::new());
        users.push(user);
        if save_users(&users).is_err() {
            eprintln!("Failed to save users.");
        } else {
            println!("User added successfully.");
        }
    } else {
        println!("User addition canceled.");
    }
    display_divider();
}

pub fn switch_user() {
    let users = load_users().unwrap_or_else(|err| {
        eprintln!("Error loading users: {}", err);
        Vec::new()
    });

    if users.is_empty() {
        println!("No users available to switch.");
        return;
    }

    let user_names: Vec<String> = users
        .iter()
        .map(|user| format!("{} <{}>", user.name, user.email))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a user to switch to")
        .items(&user_names)
        .default(0)
        .interact()
        .unwrap();

    let selected_user = &users[selection];
    let command = format!(
        "git config --global user.name \"{}\" && git config --global user.email \"{}\"",
        selected_user.name, selected_user.email
    );

    run_command(&command);
    println!(
        "Switched to User: Name: {}, Email: {}",
        selected_user.name, selected_user.email
    );
}

fn load_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let path = get_users_file_path();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let users = serde_json::from_reader(reader)?;
    Ok(users)
}

fn save_users(users: &[User]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_users_file_path();
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &users)?;
    Ok(())
}