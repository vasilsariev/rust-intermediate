use std::{
    io::{Error, ErrorKind},
    process::Command,
};

fn os_commands_example() {
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

fn ls_command() {
    println!("\n\n");
    let mut cmd_root = Command::new("ls");
    cmd_root.status().expect("Failed to execute list command!");
    println!("\n\n");
    cmd_root
        .current_dir("src")
        .status()
        .expect("Failed to execute list command.");
}

//error handling exercise
fn error_handling(dir: &str) {
    println!("\n");
    let mut list_cmd = Command::new("ls");

    match list_cmd.current_dir(dir).status() {
        Ok(cmd) => Some(cmd),
        Err(e) => {
            println!("{}", e);
            None
        }
    };
    // list_cmd.current_dir(dir).status().expect("Failed to execute list command");
    //     match list_cmd.current_dir(dir).status() {
    //         Ok(cmd) => cmd,
    //         Err(e) => panic!("Error: {}", e),
    //     };
}

fn error_handling_advanced(file: &str) {
    println!("\n");
    println!("The file printing function!!!");

    let mut print_file = Command::new("cat");

    match print_file.arg(file).status() {
        Ok(cmd) => Some(cmd),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("File not found!");
                None
            }
            _ => panic!("An unexpected error has occured!"),
        },
    };

    // print_file.arg(file).status().unwrap();
}

fn error_handling_question_mark(dir: &str) -> Result<i32, Error> {
    println!("\n");

    let mut list_cmd = Command::new("ls");

    list_cmd.current_dir(dir).status()?;

    println!("\n");
    Ok(1)
}

fn main() {
    error_handling_question_mark("src");
    error_handling_question_mark("lib");
    error_handling_advanced("Cargo.toml");
    error_handling("src");
    error_handling("lib");
    os_commands_example();
    ls_command();
}
