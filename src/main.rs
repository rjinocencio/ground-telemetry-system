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

    println!("==GROUND TELEMETRY PROCESSING SYSTEM ==");
    status(&spacecraft);
    println!("{} is in {} mode", spacecraft.identifier, spacecraft.mode);
}

fn status(spacecraft: &Spacecraft) {
    println!("Spacecraft Identifier: {}", spacecraft.identifier);
    println!("Mode: {}", spacecraft.mode);
    println!("Battery Level: {:.1}V", spacecraft.battery_voltage);
    println!("Temperature: {:.1}F", spacecraft.temperature);
    println!("Uptime: {:.1} hrs", spacecraft.uptime)
}
