use std::process::{Child, ChildStdout, Command};

fn main() {
    let mut command: Command = Command::new("cat");
    command.arg("/etc/hosts");

    let mut child: Child = command.spawn().unwrap();
    child.wait().unwrap();
}
