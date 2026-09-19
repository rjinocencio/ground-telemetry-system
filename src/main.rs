mod spacecraft;

use crate::spacecraft::{Spacecraft, SpacecraftMode};
use std::io::{self, Write};

fn main() {
    let mut spacecraft = Spacecraft {
        identifier: String::from("SAT-001"),
        mode: SpacecraftMode::Nominal,
        battery_voltage: 28.5,
        temperature: 68.0,
        uptime: 2.0,
    };

    println!("== GROUND TELEMETRY PROCESSING SYSTEM ==");
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input = input.trim();
        let command = command_parser(input);

        match command {
            Command::Status => spacecraft.print_status(),
            Command::Help => print_help(),
            Command::Nominal => spacecraft.set_mode(SpacecraftMode::Nominal),
            Command::Safe => spacecraft.set_mode(SpacecraftMode::Safe),
            Command::Standby => spacecraft.set_mode(SpacecraftMode::Standby),
            Command::Exit => {
                println!("Exiting application...");
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
==========================
"
    )
}

fn command_parser(input: &str) -> Command {
    match input {
        "status" => Command::Status,
        "help" | "?" => Command::Help,
        "nominal" => Command::Nominal,
        "safe" => Command::Safe,
        "standby" => Command::Standby,
        "exit" | "quit" => Command::Exit,
        "" => Command::Empty,
        _ => Command::Invalid,
    }
}

enum Command {
    Status,
    Help,
    Exit,
    Nominal,
    Safe,
    Standby,
    Empty,
    Invalid,
}
