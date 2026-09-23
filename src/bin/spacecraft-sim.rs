use gtps::spacecraft::{Spacecraft, SpacecraftMode};
use gtps::telemetry::TelemetrySnapshot;

use std::io::{self, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{env, thread};

const DEFAULT_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let address = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(DEFAULT_ADDRESS));

    let spacecraft = Arc::new(Spacecraft {
        identifier: String::from("SAT-001"),
        mode: SpacecraftMode::Nominal,
        battery_voltage: 28.5,
        temperature: 68.0,
        started_at: Instant::now(),
    });

    println!("== SPACECRAFT SIMULATOR ==");
    println!(
        "{} initialized in {:?} mode",
        spacecraft.identifier, spacecraft.mode
    );

    let listener = match bind_listener(&address) {
        Ok(listener) => listener,

        Err(error) => {
            println!("Failed to bind to {address}: {error}");
            return;
        }
    };

    println!("Listening on {address}");

    loop {
        match accept_connection(&listener) {
            Ok((stream, peer_address)) => {
                println!("Ground station connected from {peer_address}");
                let spacecraft = Arc::clone(&spacecraft);

                thread::spawn(move || stream_telemetry(stream, spacecraft));
            }

            Err(error) => {
                println!("Failed to accept connection: {error}");
            }
        }
    }
}

fn bind_listener(address: &str) -> io::Result<TcpListener> {
    TcpListener::bind(address)
}

fn accept_connection(listener: &TcpListener) -> io::Result<(TcpStream, SocketAddr)> {
    listener.accept()
}

fn stream_telemetry(mut stream: TcpStream, spacecraft: Arc<Spacecraft>) {
    loop {
        if let Err(error) = send_telemetry(&mut stream, &spacecraft) {
            println!("Telemetry connection lost: {error}");
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn send_telemetry(stream: &mut TcpStream, spacecraft: &Spacecraft) -> io::Result<()> {
    let telemetry = TelemetrySnapshot::from_spacecraft(spacecraft);

    let json = serde_json::to_string(&telemetry).map_err(io::Error::other)?;

    writeln!(stream, "{json}")?;

    Ok(())
}

/* Test */

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    #[test]
    fn binds_tcp_listener() {
        let listener = bind_listener("127.0.0.1:0").expect("Failed to bind test listener");

        let address = listener
            .local_addr()
            .expect("Failed to get listener address");

        assert!(address.ip().is_loopback());
        assert_ne!(address.port(), 0);
    }

    #[test]
    fn returns_error_for_invalid_bind_address() {
        let result = bind_listener("not-a-valid-address");

        assert!(result.is_err());
    }

    #[test]
    fn accepts_ground_station_connection() {
        let listener = bind_listener("127.0.0.1:0").expect("Failed to bind test listener");

        let address = listener
            .local_addr()
            .expect("Failed to get listener address");

        let client = std::thread::spawn(move || {
            TcpStream::connect(address).expect("Ground station failed to connect");
        });

        let (_stream, peer_address) =
            accept_connection(&listener).expect("Failed to accept connection");

        assert!(peer_address.ip().is_loopback());

        client.join().expect("Client thread failed");
    }

    #[test]
    fn sends_newline_delimited_telemetry_over_tcp() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind test listener");

        let address = listener
            .local_addr()
            .expect("Failed to get test listener address");

        let spacecraft = Spacecraft {
            identifier: String::from("SAT-001"),
            mode: SpacecraftMode::Nominal,
            battery_voltage: 28.5,
            temperature: 68.0,
            started_at: Instant::now(),
        };

        let sender = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("Failed to accept test connection");

            send_telemetry(&mut stream, &spacecraft).expect("Failed to send test telemetry");
        });

        let stream = TcpStream::connect(address).expect("Failed to connect test ground station");

        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        let bytes_read = reader
            .read_line(&mut line)
            .expect("Failed to read telemetry");

        assert!(bytes_read > 0);
        assert!(line.ends_with('\n'));

        let telemetry: TelemetrySnapshot =
            serde_json::from_str(line.trim_end()).expect("Failed to deserialize telemetry");

        assert_eq!(telemetry.spacecraft_id, "SAT-001");
        assert_eq!(telemetry.mode, SpacecraftMode::Nominal);
        assert_eq!(telemetry.battery_voltage, 28.5);
        assert_eq!(telemetry.temperature, 68.0);

        sender.join().expect("Telemetry sender thread failed");
    }
}
