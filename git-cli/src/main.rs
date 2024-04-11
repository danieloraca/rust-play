use std::process::{Command, Stdio};

const CHATGPT_QUESTION: &str =
    "From the above git diff describe the changes in less than 10 words as a commit message";

fn fire_command(command_cli: &str, command_arguments: &str) -> String {
    let mut command: Command = Command::new(command_cli);
    command.arg(command_arguments);

    let command_result = command.stdout(Stdio::piped()).output().unwrap();

    String::from_utf8(command_result.stdout).unwrap()
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
    let final_question: String = format!("{} {}", concatenated_lines.to_string(), CHATGPT_QUESTION);

    let mut chat_command: Command = Command::new("/Users/danieloraca/git-cli/rust-chatgpt-cli");
    chat_command.arg(final_question);

    let result = chat_command.stdout(Stdio::piped()).output().unwrap();
    let chatgpt_commit_comment: String = String::from_utf8(result.stdout).unwrap();
    println!("commit -am \"{}\"", chatgpt_commit_comment.trim());

    let commit_arg: String = format!("commit -am \"{}\"", chatgpt_commit_comment.trim());

    fire_command("/usr/bin/git", &commit_arg);
    fire_command("/usr/bin/git", "push");
}
