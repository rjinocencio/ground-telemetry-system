use gtps::spacecraft::{Spacecraft, SpacecraftMode};
use gtps::telemetry::TelemetrySnapshot;

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let spacecraft = Spacecraft {
        identifier: String::from("SAT-001"),
        mode: SpacecraftMode::Nominal,
        battery_voltage: 28.5,
        temperature: 68.0,
        started_at: Instant::now(),
    };

    println!("== SPACECRAFT SIMULATOR ==");
    println!(
        "{} initialized in {:?} mode",
        spacecraft.identifier, spacecraft.mode
    );

    for _ in 0..5 {
        let telemetry = TelemetrySnapshot::from_spacecraft(&spacecraft);

        let json = serde_json::to_string(&telemetry).expect("Failed to serialize telemetry");

        println!("{json}");

        thread::sleep(Duration::from_secs(1));
    }
}
