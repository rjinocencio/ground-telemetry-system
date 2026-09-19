pub struct Spacecraft {
    pub identifier: String,
    pub mode: SpacecraftMode,
    pub battery_voltage: f64,
    pub temperature: f64,
    pub uptime: f64,
}

#[derive(Debug, PartialEq)]
pub enum SpacecraftMode {
    Nominal,
    Safe,
    Standby,
}

impl Spacecraft {
    pub fn print_status(&self) {
        let mode = match self.mode {
            SpacecraftMode::Nominal => "Nominal",
            SpacecraftMode::Safe => "Safe",
            SpacecraftMode::Standby => "Standby",
        };

        println!("Spacecraft Identifier: {}", self.identifier);
        println!("Mode: {}", mode);
        println!("Battery Level: {:.1}V", self.battery_voltage);
        println!("Temperature: {:.1}F", self.temperature);
        println!("Uptime: {:.1} hrs", self.uptime)
    }

    pub fn set_mode(&mut self, mode: SpacecraftMode) {
        if self.mode == mode {
            println!("Spacecraft already on {:?} mode!", mode);
            return;
        }

        println!("== Previous Status ==");
        self.print_status();

        self.mode = mode;

        println!("===  New Status  ===");
        self.print_status();
        println!("=================");
    }
}
