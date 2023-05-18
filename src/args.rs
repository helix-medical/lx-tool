//! # Arguments Module
//! The argument parser for the Helix tool
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub struct Args {
    /// The command to run
    /// [`start`, `stop`, `restart`, `status`, `logs`, `clean`, `config`, `install`]
    #[clap(forbid_empty_values = true, validator = validate_command)]
    pub command: String,

    /// Sets dev mode
    #[clap(long)]
    pub dev: bool,
}

/// Validates the command provided by the user
/// # Arguments
/// * `command` - The command to validate
/// # Returns
/// * `Result<(), String>` - Ok if the command is valid, Err otherwise
/// # Example
/// ```
/// use tool::args::validate_command;
///     assert_eq!(validate_command("start"), Ok(()));
///     assert_eq!(validate_command("stop"), Ok(()));
///     assert_eq!(validate_command("restart"), Ok(()));
///     assert_eq!(validate_command("status"), Ok(()));
///     assert_eq!(validate_command("logs"), Ok(()));
///     assert_eq!(validate_command("clean"), Ok(()));
///     assert_eq!(validate_command("config"), Ok(()));
///     assert_eq!(validate_command("install"), Ok(()));
///     assert_eq!(validate_command("invalid"), Err(String::from("Command doesn't exist.")));
/// ```
fn validate_command(command: &str) -> Result<(), String> {
    let commands: Vec<&str> = vec![
        "start", "stop", "restart", "status", "logs", "clean", "config", "install",
    ];
    if !commands.contains(&command) {
        Err(String::from("Command doesn't exist."))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("start", Ok(()); "start")]
    #[test_case("stop", Ok(()); "stop")]
    #[test_case("restart", Ok(()); "restart")]
    #[test_case("status", Ok(()); "status")]
    #[test_case("logs", Ok(()); "logs")]
    #[test_case("clean", Ok(()); "clean")]
    #[test_case("config", Ok(()); "config")]
    #[test_case("install", Ok(()); "install")]
    #[test_case("invalid", Err(String::from("Command doesn't exist.")); "invalid")]
    fn test_validate_command(command: &str, result: Result<(), String>) {
        assert_eq!(validate_command(command), result);
    }
}
