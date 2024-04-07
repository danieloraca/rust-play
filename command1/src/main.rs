use colored::*;
use std::process::{Command, Stdio};

macro_rules! print_line {
    ($line:expr) => {{
        let msg = $line.blue().on_yellow();
        println!("{msg}");
        println!("{}", "-".repeat(get_screen_size()));
    }};
}

fn get_screen_size() -> usize {
    let mut width = 0;
    if let Some((x, _)) = term_size::dimensions() {
        width = x;
    }

    width
}

fn main() {
    let mut command: Command = Command::new("ls");
    command.arg("-la");

    let output = Command::new("ls")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    stdout.lines().for_each(|line| print_line!(line));
}
