use std::{
    process::{Command, Stdio},
    sync::Arc,
};

const CHATGPT_QUESTION: &str =
    "\nPlease make in less than 15 words a commit message from the above diff.";

const GIT_LOCATION: &str = "/usr/bin/git";
const GIT_DIFF: &str = "diff";
const GIT_PUSH: &str = "push";
const CHATGPT_CLI: &str = "/Users/danieloraca/git-cli/rust-chatgpt-cli";

fn fire_command(command_cli: Box<str>, command_arguments: Box<str>) -> Arc<str> {
    let mut command: Command = Command::new(&*command_cli);
    command.arg(&*command_arguments);

    let command_result = command.stdout(Stdio::piped()).output().unwrap();

    let result_str: String = String::from_utf8(command_result.stdout).unwrap();
    Arc::from(result_str.trim())
}

fn talk_to_chatgpt(concatenated_lines: Arc<str>) -> Arc<str> {
    let final_question: String = format!("{} {}", concatenated_lines, CHATGPT_QUESTION);

    fire_command(CHATGPT_CLI.into(), final_question.into())
}

fn read_git_diff() -> Arc<str> {
    let mut lines: Vec<String> = vec![];

    let diff_output: Arc<str> = fire_command(GIT_LOCATION.into(), GIT_DIFF.into());
    diff_output.lines().for_each(|line| {
        lines.push(line.to_string());
    });

    let concatenated_lines = lines.join("\n");

    Arc::from(&*concatenated_lines)
}

fn main() {
    let concatenated_lines: Arc<str> = read_git_diff();

    if concatenated_lines.is_empty() {
        println!("Nohing to commit");
        std::process::exit(0);
    }

    let chatgpt_commit_message = talk_to_chatgpt(concatenated_lines);

    println!("commit -am \"{}\"", chatgpt_commit_message.trim());

    let commit_arg: String = format!("commit -am \"{}\"", chatgpt_commit_message.trim());

    let commit_output: Arc<str> = fire_command(GIT_LOCATION.into(), commit_arg.into());

    commit_output.lines().for_each(|line| {
        println!("line is {line}");
    });

    let push = fire_command(GIT_LOCATION.into(), GIT_PUSH.into());
    println!("{push}");

    let status_output: Arc<str> = fire_command(GIT_LOCATION.into(), "status".into());
    println!("Git Status Output: {}", status_output);

    let ls_output: Arc<str> = fire_command("ls".into(), "-la".into());
    ls_output.lines().for_each(|line| {
        println!("{line}");
    })
}
