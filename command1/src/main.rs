use colored::*;
use std::{
    i32,
    process::{Command, Stdio},
};

macro_rules! print_line {
    ($line:expr) => {{
        let parts: Vec<&str> = $line.split_whitespace().collect();
        // for part in parts {
        //     print!("{part}||");
        // }
        let msg = $line.white().on_black();
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

    let output = command
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    stdout.lines().for_each(|line| print_line!(line));

    let int_number: u32 = 50048 - 49856;
    if let Some(the_char) = std::char::from_u32(int_number) {
        println!("{}", the_char);
    } else {
        println!("sucks");
    }
}
