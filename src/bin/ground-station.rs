use gtps::command::Command;

use std::io::{self, Write};

fn main() {
    println!("== GROUND STATION ==");
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input = input.trim();
        let command = Command::parser(input);

        match command {
            Command::Help => print_help(),
            Command::Exit => {
                println!("Exiting ground station...");
                break;
            }
            Command::Empty => {
                println!("Please use \"help\" or \"?\" for list of commands");
            }
            Command::Invalid => {
                println!(
                    "Command not implemented!\nPlease use \"help\" or \"?\" for list of commands"
                );
            }
            Command::Status | Command::Nominal | Command::Safe | Command::Standby => {
                println!("Spacecraft communication not yet implemented!")
            }
        }
    }
}

fn print_help() {
    println!(
        r"=== AVAILABLE COMMANDS ===
help, ?      Display this help menu
status       Check system status
nominal      Sets mode to nominal
safe         Sets mode to safe
standby      Sets mode to standby
exit, quit   Exit the application
=========================="
    )
}
