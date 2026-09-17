mod spacecraft;

use crate::spacecraft::{Spacecraft, SpacecraftMode};

fn main() {
    let spacecraft = Spacecraft {
        identifier: String::from("SAT-001"),
        mode: SpacecraftMode::Nominal,
        battery_voltage: 28.5,
        temperature: 68.0,
        uptime: 2.0,
    };

    println!("== GROUND TELEMETRY PROCESSING SYSTEM ==");
    spacecraft.print_status();
    println!(" ======================================= ");
    println!("{} is in {:?} mode", spacecraft.identifier, spacecraft.mode);
}
