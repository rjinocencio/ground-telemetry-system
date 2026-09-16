pub struct Spacecraft {
    pub identifier: String,
    pub mode: String,
    pub battery_voltage: f64,
    pub temperature: f64,
    pub uptime: f64,
}

impl Spacecraft {
    pub fn status(&self) {
        println!("Spacecraft Identifier: {}", self.identifier);
        println!("Mode: {}", self.mode);
        println!("Battery Level: {:.1}V", self.battery_voltage);
        println!("Temperature: {:.1}F", self.temperature);
        println!("Uptime: {:.1} hrs", self.uptime)
    }
}
