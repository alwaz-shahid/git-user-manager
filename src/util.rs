use std::io::{self, Write};
use std::process::{Command, Output};

pub fn display_divider() {
    println!("----------------------------------------------------------------");
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Consider handling errors more gracefully
    input.trim().to_string() // Trim whitespace and return
}

pub fn run_command(cmd: &str) {
    let output: Output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", cmd])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command")
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        println!("{}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }
}
