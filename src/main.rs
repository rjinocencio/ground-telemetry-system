fn main() {
    let identifier = "SAT-001";
    let mode = "NOMINAL";
    let battery_voltage = 28.5;
    let temperature = 68.0;

    println!("==GROUND TELEMETRY PROCESSING SYSTEM ==");
    println!("Spacecraft Identifier: {}", identifier);
    println!("Mode: {}", mode);
    println!("Battery Level: {:.1}V", battery_voltage);
    println!("Temperature: {:.1}F", temperature);
}
