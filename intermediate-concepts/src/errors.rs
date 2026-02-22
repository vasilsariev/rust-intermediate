use std::{
    io::{Error, ErrorKind},
    process::Command,
};

pub fn list_files(dir: &str) {
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

pub fn matching(file: &str) {
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

pub fn question_mark(dir: &str) -> Result<i32, Error> {
    println!("\n");

    let mut list_cmd = Command::new("ls");

    list_cmd.current_dir(dir).status()?;

    println!("\n");
    Ok(1)
}
