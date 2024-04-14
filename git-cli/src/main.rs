use std::process::{Command, Stdio};

const CHATGPT_QUESTION: &str =
    "\nPlease make in less than 15 words a commit message from the above diff.";

const GIT_LOCATION: &str = "/usr/bin/git";
const GIT_DIFF: &str = "diff";
const GIT_PUSH: &str = "push";
const CHATGPT_CLI: &str = "/Users/danieloraca/git-cli/rust-chatgpt-cli";

fn fire_command(command_cli: Box<str>, command_arguments: Box<str>) -> String {
    let mut command: Command = Command::new(command_cli.to_string());
    command.arg(command_arguments.to_string());

    let command_result = command.stdout(Stdio::piped()).output().unwrap();

    String::from_utf8(command_result.stdout).unwrap()
}

fn talk_to_chatgpt(concatenated_lines: String) -> String {
    let final_question: String = format!("{} {}", concatenated_lines.to_string(), CHATGPT_QUESTION);

    let chat_response = fire_command(CHATGPT_CLI.into(), final_question.into());

    chat_response
}

fn read_git_diff() -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    let diff_output: String = fire_command(GIT_LOCATION.into(), GIT_DIFF.into());
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

    let commit_output: String = fire_command(GIT_LOCATION.into(), commit_arg.into());

    commit_output.lines().for_each(|line| {
        println!("line is {line}");
    });

    let push = fire_command(GIT_LOCATION.into(), GIT_PUSH.into());
    println!("{push}");

    let status_output: String = fire_command(GIT_LOCATION.into(), "status".into());
    println!("Git Status Output: {}", status_output);

    let ls_output: String = fire_command("ls".into(), "-la".into());
    ls_output.lines().for_each(|line| {
        println!("{line}");
    })
}
