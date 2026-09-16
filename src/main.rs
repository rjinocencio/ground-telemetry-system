mod spacecraft;

use crate::spacecraft::Spacecraft;

fn main() {
    let spacecraft = Spacecraft {
        identifier: String::from("SAT-001"),
        mode: String::from("NOMINAL"),
        battery_voltage: 28.5,
        temperature: 68.0,
        uptime: 2.0,
    };

    println!("== GROUND TELEMETRY PROCESSING SYSTEM ==");
    spacecraft.status();
    println!(" ======================================= ");
    println!("{} is in {} mode", spacecraft.identifier, spacecraft.mode);
}
