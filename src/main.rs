//! > **The Helix Utility Tool**
//!
//! The Helix Utility Tool is a command line tool that allows you to manage the Helix Server.
//! ## Usage
//! ```bash
//! lx [FLAGS] <COMMAND>
//! ```
//! ## Flags
//! * `-h`, `--help` - Prints help information
//! * `-V`, `--version` - Prints version information
//! * `--dev` - Sets dev mode
//! ## Commands
//! * `start` - Starts the Helix Server
//! * `stop` - Stops the Helix Server
//! * `restart` - Restarts the Helix Server
//! * `status` - Prints the status of the Helix Server
//! * `logs` - Prints the logs of the Helix Server
//! * `clean` - Cleans the Helix Server
//!
//! ## Examples
//! ```bash
//! lx start
//! lx stop
//! ```
//!
//! ## Installation
//! ```bash
//! git clone <REPO_URL> && cd lx-tool
//! cargo build --release
//! ```
//!
//! ## Add to PATH
//! ```bash
//! sudo cp target/release/lx /usr/local/bin
//! ```
mod args;
mod commands;
mod config;
mod docker;
mod install;
use clap::Parser;
use std::process::exit;

/// Prints the header of the tool
/// # Arguments
/// * `is_dev` - Whether the tool is running in dev mode
/// # Example
/// ```
/// use tool::header;
/// header(true);
/// ```
/// # Output
/// ```text
/// -- The Helix Utility Tool --
/// --   Credits:  Xavier2p   --
/// --  󰗦 Helix Medical 2023  --
/// --    Environment: Dev    --
/// ```
#[allow(unused_variables)]
fn header(is_dev: bool) {
    println!(" -- The Helix Utility Tool --");
    println!(" --   Credits:  Xavier2p   --");
    println!(" --  󰗦 Helix Medical 2023  --\n");
    // println!(
    //     " --    Environment: {}   -- ",
    //     if is_dev { "Dev " } else { "Prod" }
    // )
}

/// The main function of the tool
/// # Example
/// ```
/// use tool::main;
/// main();
/// ```
/// # Output
/// ```text
/// -- The Helix Utility Tool --
/// --   Credits:  Xavier2p   --
/// --  󰗦 Helix Medical 2023  --
/// --    Environment: Dev    --
/// ```
fn main() {
    let args::Args { command, dev }: args::Args = args::Args::parse();
    header(dev);

    match command.as_str() {
        "start" => commands::start(dev),
        "stop" => commands::stop(dev),
        "install" => install::all(),
        "restart" => println!(" -- Restarting Helix --"),
        "status" => println!(" -- Helix Status --"),
        "logs" => println!(" -- Helix Logs --"),
        "config" => config::create_config().expect("Failed to create config file"),
        "clean" => commands::clean(dev),
        _ => {
            println!("Invalid arguments provided. Please use `lx -h` for more information.");
            exit(1);
        }
    }
}
