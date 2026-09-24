use gtps::command::Command;
use gtps::telemetry::TelemetrySnapshot;

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    println!("== GROUND STATION ==");

    let mut input = String::new();
    let connections: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

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

                println!("Connecting to {address}...");

                match connect_spacecraft(&address, &connections) {
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
        r"
========= AVAILABLE COMMANDS =========
connect [ip:port]        Connect to a spacecraft
connections              List currently retained spacecraft connections
help, ?                  Display this help menu
status                   Check system status
nominal                  Sets mode to nominal
safe                     Sets mode to safe
standby                  Sets mode to standby
exit, quit               Exit the application
======================================"
    )
}

fn connect_spacecraft(
    address: &str,
    connections: &Arc<Mutex<HashMap<String, TcpStream>>>,
) -> std::io::Result<bool> {
    {
        let connections = connections
            .lock()
            .expect("Connection registry lock poisoned");

        if connections.contains_key(address) {
            return Ok(false);
        }
    }

    let stream = TcpStream::connect(address)?;
    let receiver_stream = stream.try_clone()?;

    {
        let mut connections = connections
            .lock()
            .expect("Connection registry lock poisoned");

        connections.insert(address.to_string(), stream);
    }

    let connections = Arc::clone(connections);
    let receiver_address = address.to_string();

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
    let mut last_sequence: Option<u64> = None;

    loop {
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {
                let timestamp_ms = get_timestamp_ms();

                println!("[{timestamp_ms}] Spacecraft at {address} disconnected.");
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
                    if let Some(expected) =
                        sequence_warning(last_sequence, telemetry.sequence_number)
                    {
                        let timestamp_ms = get_timestamp_ms();

                        println!(
                            "[{timestamp_ms}] Telemetry sequence warning from {}: expected {}, received {}",
                            telemetry.spacecraft_id, expected, telemetry.sequence_number,
                        );
                    }

                    last_sequence = Some(telemetry.sequence_number);

                    println!(
                        "[{}] seq={} | {:?} | {:.1} V | {:.1} F | {:.1}s | {}",
                        telemetry.spacecraft_id,
                        telemetry.sequence_number,
                        telemetry.mode,
                        telemetry.battery_voltage,
                        telemetry.temperature,
                        telemetry.uptime_seconds,
                        telemetry.timestamp_ms
                    );
                }

                Err(error) => {
                    let timestamp_ms = get_timestamp_ms();

                    println!("[{timestamp_ms}] Invalid telemetry from {address}: {error}");
                }
            },

            Err(error) => {
                let timestamp_ms = get_timestamp_ms();

                println!("[{timestamp_ms}] Telemetry read error from {address}: {error}");
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

fn get_timestamp_ms() -> u64 {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before Unix epoch")
            .as_millis(),
    )
    .expect("Timestamp exceeds u64 range")
}

fn sequence_warning(previous: Option<u64>, current: u64) -> Option<u64> {
    match previous {
        Some(previous) => {
            let expected = previous + 1;

            if current != expected {
                Some(expected)
            } else {
                None
            }
        }

        None => None,
    }
}

/* Tests */
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn connects_to_spacecraft() {
        use std::sync::mpsc;

        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");

        let address = listener
            .local_addr()
            .expect("Failed to get listener address");

        let address_string = address.to_string();

        let (release_tx, release_rx) = mpsc::channel();

        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().expect("Failed to accept test connection");

            release_rx
                .recv()
                .expect("Failed to wait for test completion");

            drop(stream);
        });

        let connections = Arc::new(Mutex::new(HashMap::new()));

        let result = connect_spacecraft(&address_string, &connections).expect("Failed to connect");

        {
            let connections = connections
                .lock()
                .expect("Connection registry lock poisoned");

            assert!(result);
            assert!(connections.contains_key(&address_string));
        }

        release_tx.send(()).expect("Failed to release test server");

        server.join().expect("Server thread failed");
    }
    #[test]
    fn removes_connection_when_spacecraft_disconnects() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");

        let server_address = listener
            .local_addr()
            .expect("Failed to get test listener address");

        let address = server_address.to_string();

        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().expect("Failed to accept test connection");

            drop(stream);
        });

        let stream =
            TcpStream::connect(server_address).expect("Failed to connect to test spacecraft");

        let receiver_stream = stream.try_clone().expect("Failed to clone test stream");

        let connections = Arc::new(Mutex::new(HashMap::<String, TcpStream>::new()));

        {
            let mut connections = connections
                .lock()
                .expect("Connection registry lock poisoned");

            connections.insert(address.clone(), stream);
        }

        let receiver_connections = Arc::clone(&connections);
        let receiver_address = address.clone();

        let receiver = thread::spawn(move || {
            receive_telemetry(receiver_stream, receiver_address, receiver_connections);
        });

        server.join().expect("Test spacecraft thread failed");

        receiver.join().expect("Telemetry receiver thread failed");

        let connections = connections
            .lock()
            .expect("Connection registry lock poisoned");

        assert!(!connections.contains_key(&address));
    }

    #[test]
    fn accepts_next_sequence_number() {
        assert_eq!(sequence_warning(Some(10), 11), None);
    }

    #[test]
    fn detects_missing_sequence_number() {
        assert_eq!(sequence_warning(Some(10), 12), Some(11));
    }

    #[test]
    fn first_sequence_has_no_expected_predecessor() {
        assert_eq!(sequence_warning(None, 0), None);
    }

    #[test]
    fn rejects_duplicate_spacecraft_connection() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");

        let address = listener
            .local_addr()
            .expect("Failed to get listener address");

        let address_string = address.to_string();

        let connections = Arc::new(Mutex::new(HashMap::new()));

        let first =
            connect_spacecraft(&address_string, &connections).expect("First connection failed");

        let second =
            connect_spacecraft(&address_string, &connections).expect("Duplicate check failed");

        assert!(first);
        assert!(!second);

        let connections = connections
            .lock()
            .expect("Connection registry lock poisoned");

        assert_eq!(connections.len(), 1);
    }
}
