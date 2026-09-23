use gtps::command::Command;
use gtps::telemetry::TelemetrySnapshot;

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

const DEFAULT_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    println!("== GROUND STATION ==");

    let mut input = String::new();
    let mut connections: Arc<Mutex<HashMap<String, TcpStream>>> =
        Arc::new(Mutex::new(HashMap::new()));

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input = input.trim();

        let command = Command::parser(input);

        match command {
            Command::Help => print_help(),
            Command::Exit => {
                println!("Exiting ground station...");
                break;
            }
            Command::Empty => {
                println!("Please use \"help\" or \"?\" for list of commands");
            }
            Command::Invalid => {
                println!(
                    "Command not implemented!\nPlease use \"help\" or \"?\" for list of commands"
                );
            }
            Command::Connect(address) => {
                let address = address.unwrap_or_else(|| String::from(DEFAULT_ADDRESS));

                {
                    let connections = connections
                        .lock()
                        .expect("Connection registry lock poisoned");

                    if connections.contains_key(&address) {
                        println!("{address} already connected");
                        continue;
                    }
                }

                println!("Connecting to {address}");

                match connect_spacecraft(&address, &mut connections) {
                    Ok(true) => {
                        println!("Connected to spacecraft at {address}!");
                    }

                    Ok(false) => {
                        println!("{address} already connected!");
                    }

                    Err(error) => {
                        println!("Failed to connect to spacecraft: {error}");
                    }
                }
            }

            Command::Connections => {
                let connections = connections
                    .lock()
                    .expect("Connection registry lock poisoned");

                if connections.is_empty() {
                    println!("No active spacecraft connections.");
                } else {
                    println!("== ACTIVE CONNECTIONS ==");

                    for address in connections.keys() {
                        println!("{address}");
                    }
                }
            }

            Command::Status | Command::Nominal | Command::Safe | Command::Standby => {
                println!("Spacecraft communication not yet implemented!")
            }
        }
    }
}

fn print_help() {
    println!(
        r"=== AVAILABLE COMMANDS ===
connect      Connect to a spacecraft
connections  List currently retained spacecraft connections
help, ?      Display this help menu
status       Check system status
nominal      Sets mode to nominal
safe         Sets mode to safe
standby      Sets mode to standby
exit, quit   Exit the application
=========================="
    )
}

fn connect_spacecraft(
    address: &String,
    connections: &mut Arc<Mutex<HashMap<String, TcpStream>>>,
) -> std::io::Result<bool> {
    let stream = TcpStream::connect(&address)?;
    let receiver_stream = stream.try_clone()?;

    {
        let mut connections = connections
            .lock()
            .expect("Connection registry lock poisoned");
        connections.insert(address.to_string(), stream);
    }

    let connections = Arc::clone(&connections);
    let receiver_address = address.clone();

    thread::spawn(move || {
        receive_telemetry(receiver_stream, receiver_address, connections);
    });

    Ok(true)
}

fn receive_telemetry(
    stream: TcpStream,
    address: String,
    connections: Arc<Mutex<HashMap<String, TcpStream>>>,
) {
    let mut reader = BufReader::new(stream);

    loop {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("Spacecraft disconnected.");
                {
                    let mut connections = connections
                        .lock()
                        .expect("Connection registry lock poisoned");
                    connections.remove(&address);
                }
                break;
            }

            Ok(_) => match serde_json::from_str::<TelemetrySnapshot>(&line) {
                Ok(telemetry) => {
                    println!(
                        "[{}] {:?} | {:.1} V | {:.1} F | {:.1}s",
                        telemetry.spacecraft_id,
                        telemetry.mode,
                        telemetry.battery_voltage,
                        telemetry.temperature,
                        telemetry.uptime_seconds
                    );
                }

                Err(error) => {
                    println!("Invalid telemetry: {error}");
                }
            },

            Err(error) => {
                println!("Telemetry read error: {error}");
                {
                    let mut connections = connections
                        .lock()
                        .expect("Connection registry lock poisoned");
                    connections.remove(&address);
                }
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn connects_to_spacecraft() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");

        let address = listener
            .local_addr()
            .expect("Failed to get listener address");

        let server = thread::spawn(move || {
            listener.accept().expect("Failed to accept test connection");
        });

        let mut connections = Arc::new(Mutex::new(HashMap::new()));

        let result =
            connect_spacecraft(&address.to_string(), &mut connections).expect("Failed to connect");

        let connections = connections
            .lock()
            .expect("Connection registry lock poisoned");

        assert!(result);
        assert!(connections.contains_key(&address.to_string()));

        server.join().expect("Server thread failed");
    }
}
