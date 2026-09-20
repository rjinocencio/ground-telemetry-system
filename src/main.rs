mod command;
mod spacecraft;

use crate::command::Command;
use crate::spacecraft::{Spacecraft, SpacecraftMode};
use display::{print_help, print_status};

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
        let command = Command::parser(input);

        match command {
            Command::Status => print_status(&spacecraft),
            Command::Help => print_help(),
            Command::Nominal => {
                change_mode(&mut spacecraft, SpacecraftMode::Nominal);
            }
            Command::Safe => {
                change_mode(&mut spacecraft, SpacecraftMode::Safe);
            }
            Command::Standby => {
                change_mode(&mut spacecraft, SpacecraftMode::Standby);
            }
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

mod display {
    use crate::spacecraft::Spacecraft;

    pub fn print_status(spacecraft: &Spacecraft) {
        println!("Spacecraft Identifier: {}", spacecraft.identifier);
        println!("Mode: {:?}", spacecraft.mode);
        println!("Battery Level: {:.1} V", spacecraft.battery_voltage);
        println!("Temperature: {:.1} F", spacecraft.temperature);
        println!("Uptime: {:.1} hrs", spacecraft.uptime)
    }

    pub fn print_help() {
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
}

fn change_mode(spacecraft: &mut Spacecraft, mode: SpacecraftMode) -> bool {
    if spacecraft.mode == mode {
        println!("Spacecraft already on {:?} mode!", mode);
        return false;
    }

    println!("== Previous Status ==");
    print_status(spacecraft);

    spacecraft.set_mode(mode);

    println!("===  New Status  ===");
    print_status(spacecraft);
    println!("=================");

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_mode_to_nominal() {
        let mut spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Safe,
            battery_voltage: 28.5,
            temperature: 68.0,
            uptime: 2.0,
        };
        let mode = SpacecraftMode::Nominal;

        let changed = change_mode(&mut spacecraft, mode);

        assert!(changed);
        assert_eq!(spacecraft.mode, SpacecraftMode::Nominal);
    }

    #[test]
    fn change_mode_to_safe() {
        let mut spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Nominal,
            battery_voltage: 28.5,
            temperature: 68.0,
            uptime: 2.0,
        };
        let mode = SpacecraftMode::Safe;

        let changed = change_mode(&mut spacecraft, mode);

        assert!(changed);
        assert_eq!(spacecraft.mode, SpacecraftMode::Safe);
    }

    #[test]
    fn change_mode_to_standby() {
        let mut spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Safe,
            battery_voltage: 28.5,
            temperature: 68.0,
            uptime: 2.0,
        };
        let mode = SpacecraftMode::Standby;

        let changed = change_mode(&mut spacecraft, mode);

        assert!(changed);
        assert_eq!(spacecraft.mode, SpacecraftMode::Standby);
    }

    #[test]
    fn change_mode_should_not_execute() {
        let mut spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Standby,
            battery_voltage: 28.5,
            temperature: 68.0,
            uptime: 2.0,
        };
        let mode = SpacecraftMode::Standby;

        let changed = change_mode(&mut spacecraft, mode);

        assert!(!changed);
        assert_eq!(spacecraft.mode, SpacecraftMode::Standby);
    }
}
