#[derive(Debug, PartialEq)]
pub enum Command {
    Status,
    Help,
    Exit,
    Nominal,
    Safe,
    Standby,
    Empty,
    Invalid,
}

impl Command {
    pub fn parser(input: &str) -> Command {
        match input {
            "status" => Command::Status,
            "help" | "?" => Command::Help,
            "nominal" => Command::Nominal,
            "safe" => Command::Safe,
            "standby" => Command::Standby,
            "exit" | "quit" => Command::Exit,
            "" => Command::Empty,
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
}
