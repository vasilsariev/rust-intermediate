use crate::traits::Vehicle as _;

mod errors;
mod generics;
mod os_commands;
mod traits;

fn main() {
    generics::mul_nums(1, 2);
    generics::lookup_datatype(1.9);
    // errors::question_mark("src");
    // errors::question_mark("lib");
    // errors::matching("Cargo.toml");
    // errors::list_files("src");
    // errors::list_files("lib");
    // os_commands::example();
    // os_commands::ls_command();
    let car = traits::Car {
        color: String::from("red"),
    };
    println!("{}", car.forward());
}
