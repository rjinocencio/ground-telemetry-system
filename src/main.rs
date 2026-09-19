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

    print!("\n> ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read command!");

    match input.trim() {
        "status" => spacecraft.print_status(),
        "help" | "?" => print_help(),
        "nominal" => spacecraft.set_mode("nominal"),
        "safe" => spacecraft.set_mode("safe"),
        "standby" => spacecraft.set_mode("standby"),
        _ => println!("Command not implemented!"),
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
==========================
"
    )
}
