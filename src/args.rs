//! # Arguments Module
//! The argument parser for the Helix tool
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub struct Args {
    /// The command to run
    /// [`start`, `stop`, `restart`, `status`, `logs`, `clean`, `config`]
    #[clap(forbid_empty_values = true, validator = validate_command)]
    pub command: String,

    #[clap(long)]
    /// Sets dev mode
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
///     assert_eq!(validate_command("invalid"), Err(String::from("Command doesn't exist.")));
/// ```
fn validate_command(command: &str) -> Result<(), String> {
    let commands: [&str; 7] = [
        "start", "stop", "restart", "status", "logs", "clean", "config",
    ];
    if !commands.contains(&command) {
        Err(String::from("Command doesn't exist."))
    } else {
        Ok(())
    }
}
