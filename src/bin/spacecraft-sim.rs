use gtps::spacecraft::{Spacecraft, SpacecraftMode};

use std::env;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::time::Instant;

const DEFAULT_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let address = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(DEFAULT_ADDRESS));

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

    let listener = match bind_listener(&address) {
        Ok(listener) => listener,

        Err(error) => {
            println!("Failed to bind to {address}: {error}");
            return;
        }
    };

    println!("Listening on {address}");

    let mut connections: Vec<TcpStream> = Vec::new();

    loop {
        match accept_connection(&listener, &mut connections) {
            Ok(peer_address) => {
                println!("Ground station connected from {peer_address}");
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

fn accept_connection(
    listener: &TcpListener,
    connections: &mut Vec<TcpStream>,
) -> io::Result<SocketAddr> {
    let (stream, peer_address) = listener.accept()?;

    connections.push(stream);

    Ok(peer_address)
}

#[cfg(test)]
mod tests {
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

        let mut connections = Vec::new();

        let peer_address =
            accept_connection(&listener, &mut connections).expect("Failed to accept connection");

        assert!(peer_address.ip().is_loopback());
        assert_eq!(connections.len(), 1);

        client.join().expect("Client thread failed");
    }

    #[test]
    fn accepts_multiple_ground_station_connections() {
        let listener = bind_listener("127.0.0.1:0").expect("Failed to bind test listener");

        let address = listener
            .local_addr()
            .expect("Failed to get listener address");

        let client_one = std::thread::spawn(move || {
            TcpStream::connect(address).expect("First ground station failed to connect");
        });

        let mut connections = Vec::new();

        accept_connection(&listener, &mut connections).expect("Failed to accept first connection");

        client_one.join().expect("First client thread failed");

        let client_two = std::thread::spawn(move || {
            TcpStream::connect(address).expect("Second ground station failed to connect");
        });

        accept_connection(&listener, &mut connections).expect("Failed to accept second connection");

        client_two.join().expect("Second client thread failed");

        assert_eq!(connections.len(), 2);
    }
}
