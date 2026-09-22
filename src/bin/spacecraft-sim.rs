use gtps::spacecraft::{Spacecraft, SpacecraftMode};
use gtps::telemetry::TelemetrySnapshot;

fn main() {
    let spacecraft = Spacecraft {
        identifier: String::from("SAT-001"),
        mode: SpacecraftMode::Nominal,
        battery_voltage: 28.5,
        temperature: 68.0,
        uptime: 2.0,
    };

    println!("== SPACECRAFT SIMULATOR ==");
    println!(
        "{} initialized in {:?} mode",
        spacecraft.identifier, spacecraft.mode
    );
    let telemetry = TelemetrySnapshot::from_spacecraft(&spacecraft);

    let json = serde_json::to_string(&telemetry).expect("Failed to serialize telemetry");

    println!("Telemetry: {}", json);
}
