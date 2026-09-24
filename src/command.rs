#[derive(Debug, PartialEq)]
pub enum Command {
    Status,
    Help,
    Exit,
    Nominal,
    Safe,
    Standby,
    Connect(Option<String>),
    Connections,
    Empty,
    Invalid,
}

impl Command {
    pub fn parser(input: &str) -> Command {
        let mut parts = input.split_whitespace();

        let Some(command) = parts.next() else {
            return Command::Empty;
        };

        let argument = parts.next();
        if parts.next().is_some() {
            return Command::Invalid;
        }

        match (command, argument) {
            ("status", None) => Command::Status,
            ("help" | "?", None) => Command::Help,
            ("nominal", None) => Command::Nominal,
            ("safe", None) => Command::Safe,
            ("standby", None) => Command::Standby,
            ("exit" | "quit", None) => Command::Exit,
            ("connect", address) => Command::Connect(address.map(String::from)),
            ("connections", None) => Command::Connections,
            _ => Command::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status() {
        assert_eq!(Command::parser("status"), Command::Status);
    }

    #[test]
    fn parses_help() {
        assert_eq!(Command::parser("help"), Command::Help);
    }

    #[test]
    fn parses_question_mark() {
        assert_eq!(Command::parser("?"), Command::Help);
    }

    #[test]
    fn parses_exit() {
        assert_eq!(Command::parser("exit"), Command::Exit);
    }

    #[test]
    fn parses_quit() {
        assert_eq!(Command::parser("quit"), Command::Exit);
    }

    #[test]
    fn parses_nominal_mode() {
        assert_eq!(Command::parser("nominal"), Command::Nominal);
    }

    #[test]
    fn parses_safe_mode() {
        assert_eq!(Command::parser("safe"), Command::Safe);
    }

    #[test]
    fn parses_standby_mode() {
        assert_eq!(Command::parser("standby"), Command::Standby);
    }

    #[test]
    fn parses_unknown() {
        assert_eq!(Command::parser("banana"), Command::Invalid);
    }

    #[test]
    fn parses_empty() {
        assert_eq!(Command::parser(""), Command::Empty);
    }

    #[test]
    fn parses_connect_with_address() {
        let command = Command::parser("connect 127.0.0.1:7878");

        assert_eq!(
            command,
            Command::Connect(Some(String::from("127.0.0.1:7878")))
        );
    }

    #[test]
    fn parses_connect_without_address() {
        let command = Command::parser("connect");

        assert_eq!(command, Command::Connect(None));
    }

    #[test]
    fn parses_connections() {
        let command = Command::parser("connections");

        assert_eq!(command, Command::Connections);
    }

    #[test]
    fn rejects_connect_with_too_many_arguments() {
        let command = Command::parser("connect 127.0.0.1:7878 127.0.0.1:6868");

        assert_eq!(command, Command::Invalid);
    }
}
