use std::process::{Command, Output, Stdio};

const CHATGPT_QUESTION: &str =
    "\nPlease make in less than 10 words a commit message from the above diff";

fn fire_command(command_cli: &str, command_arguments: &str) -> String {
    let mut command: Command = Command::new(command_cli);
    command.arg(command_arguments);

    let command_result = command.stdout(Stdio::piped()).output().unwrap();

    String::from_utf8(command_result.stdout).unwrap()
}

fn talk_to_chatgpt(concatenated_lines: String) -> String {
    let final_question: String = format!("{} {}", concatenated_lines.to_string(), CHATGPT_QUESTION);

    let chat_response = fire_command(
        "/Users/danieloraca/git-cli/rust-chatgpt-cli",
        &final_question,
    );

    chat_response
}

fn read_git_diff() -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    let diff_output = fire_command("/usr/bin/git", "diff");
    diff_output.lines().for_each(|line| {
        lines.push(line.to_string());
    });

    lines
}

fn main() {
    let git_diff: Vec<String> = read_git_diff();
    let concatenated_lines: String = git_diff.join("\n");

    if concatenated_lines.is_empty() {
        println!("Nohing to commit");
        std::process::exit(0);
    }

    let chatgpt_commit_message = talk_to_chatgpt(concatenated_lines);

    println!("commit -am \"{}\"", chatgpt_commit_message.trim());

    let commit_arg: String = format!("commit -am \"{}\"", chatgpt_commit_message.trim());

    fire_command("/usr/bin/git", &commit_arg);
    fire_command("/usr/bin/git", "push");
}
