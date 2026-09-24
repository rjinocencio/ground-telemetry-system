use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::spacecraft::{Spacecraft, SpacecraftMode};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub spacecraft_id: String,
    pub mode: SpacecraftMode,
    pub battery_voltage: f64,
    pub temperature: f64,
    pub uptime_seconds: f64,
    pub timestamp_ms: u64,
    pub sequence_number: u64,
}

impl TelemetrySnapshot {
    pub fn from_spacecraft(spacecraft: &Spacecraft, sequence_number: u64) -> Self {
        let timestamp_ms = u64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System time is before Unix epoch")
                .as_millis(),
        )
        .expect("Timestamp exceeds u64 range");

        Self {
            spacecraft_id: spacecraft.identifier.clone(),
            mode: spacecraft.mode,
            battery_voltage: spacecraft.battery_voltage,
            temperature: spacecraft.temperature,
            uptime_seconds: spacecraft.uptime(),
            timestamp_ms,
            sequence_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn telemetry_round_trip() {
        let spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Nominal,
            battery_voltage: 28.5,
            temperature: 68.0,
            started_at: Instant::now(),
        };

        let telemetry = TelemetrySnapshot::from_spacecraft(&spacecraft, 75);

        let json = serde_json::to_string(&telemetry).expect("Failed to serialize telemetry");

        let decoded: TelemetrySnapshot =
            serde_json::from_str(&json).expect("Failed to deserialize telemetry");

        assert_eq!(decoded, telemetry);
    }
}
