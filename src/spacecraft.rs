use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug)]
pub struct Spacecraft {
    pub identifier: String,
    pub mode: SpacecraftMode,
    pub battery_voltage: f64,
    pub temperature: f64,
    pub started_at: Instant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum SpacecraftMode {
    Nominal,
    Safe,
    Standby,
}

impl Spacecraft {
    pub fn set_mode(&mut self, mode: SpacecraftMode) {
        self.mode = mode;
    }

    pub fn uptime(&self) -> f64 {
        self.started_at.elapsed().as_secs_f64()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_mode_to_nominal() {
        let mut spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Safe,
            battery_voltage: 28.5,
            temperature: 68.0,
            started_at: Instant::now(),
        };

        spacecraft.set_mode(SpacecraftMode::Nominal);

        assert_eq!(spacecraft.mode, SpacecraftMode::Nominal);
    }

    #[test]
    fn sets_mode_to_safe() {
        let mut spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Nominal,
            battery_voltage: 28.5,
            temperature: 68.0,
            started_at: Instant::now(),
        };

        spacecraft.set_mode(SpacecraftMode::Safe);

        assert_eq!(spacecraft.mode, SpacecraftMode::Safe);
    }

    #[test]
    fn sets_mode_to_standby() {
        let mut spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Nominal,
            battery_voltage: 28.5,
            temperature: 68.0,
            started_at: Instant::now(),
        };

        spacecraft.set_mode(SpacecraftMode::Standby);

        assert_eq!(spacecraft.mode, SpacecraftMode::Standby);
    }
}
