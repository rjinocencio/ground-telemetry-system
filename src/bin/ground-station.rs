use gtps::command::Command;

use std::collections::HashMap;
use std::io::{self, Write};
use std::net::TcpStream;

const DEFAULT_ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    println!("== GROUND STATION ==");

    let mut input = String::new();
    let mut connections: HashMap<String, TcpStream> = HashMap::new();

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

                if connections.contains_key(&address) {
                    println!("{address} already connected");
                    continue;
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
    connections: &mut HashMap<String, TcpStream>,
) -> std::io::Result<bool> {
    if connections.contains_key(address) {
        return Ok(false);
    }

    let stream = TcpStream::connect(&address)?;

    connections.insert(address.to_string(), stream);

    Ok(true)
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

        let mut connections = HashMap::new();

        let result =
            connect_spacecraft(&address.to_string(), &mut connections).expect("Failed to connect");

        assert!(result);
        assert!(connections.contains_key(&address.to_string()));

        server.join().expect("Server thread failed");
    }
}
