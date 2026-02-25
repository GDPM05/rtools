
use colored::Colorize;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // Removing the first argument "target\debug\rtools.exe"
    args.remove(0);

    if args.len() < 1 {
        println!("{}", "Please specify the desired operation.".red());
        return ();
    }

    if let Some(command) = args.get(0) {
        match command.as_str() {
            "echo" => {
            },
            "cat" => {

            },
            "ls" => {

            },
            "find" => {

            },
            "grep" => {

            },
            _ => {
                println!("{}", "Invalid Operation!".red());
            }
        }
    }
}

