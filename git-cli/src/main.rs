use std::process::{Command, Stdio};

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
    let chatgpt_question: &str =
        "Can you make a pr commit message from the above git diff? No more than 15 words. Just the commit message, nothing extra. If no diff, return this text: NO CHANGES";
    let final_question: String = format!("{} {}", concatenated_lines.to_string(), chatgpt_question);

    let mut chat_command: Command = Command::new("/Users/danieloraca/git-cli/rust-chatgpt-cli");
    chat_command.arg(final_question);

    let result = chat_command.stdout(Stdio::piped()).output().unwrap();
    let stdout = String::from_utf8(result.stdout).unwrap();

    stdout.lines().for_each(|line| {
        println!("{line}");
    });
}
