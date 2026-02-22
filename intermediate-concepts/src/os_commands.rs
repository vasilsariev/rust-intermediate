use std::process::Command;

pub fn example() {
    let echo_cmd = if cfg!(target_os = "macos") {
        Command::new("sh")
            .args(["-c", "echo Hello from a macos device"])
            .output()
            .expect("Failed to execute")
    } else if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo Hello from Windows!"])
            .output()
            .expect("Failed to execute")
    } else {
        Command::new("sh")
            .args(["-c", "echo Should be linux"])
            .output()
            .expect("Failed to execute")
    };
    println!("\n\n");
    let cmd_output = String::from_utf8(echo_cmd.stdout).expect("Could not parse byte response");
    println!("{}", cmd_output);
    println!("\n\n");
}

pub fn ls_command() {
    println!("\n\n");
    let mut cmd_root = Command::new("ls");
    cmd_root.status().expect("Failed to execute list command!");
    println!("\n\n");
    cmd_root
        .current_dir("src")
        .status()
        .expect("Failed to execute list command.");
}
