mod spacecraft;

use crate::spacecraft::{Spacecraft, SpacecraftMode};
use std::io::{self, Write};

fn main() {
    let spacecraft = Spacecraft {
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
        _ => println!("Command not implemented!"),
    }
}

fn print_help() {
    print!(
        r"=== AVAILABLE COMMANDS ===
  help, ?      Display this help menu
  status       Check system status
  exit, quit   Exit the application
==========================
"
    )
}
