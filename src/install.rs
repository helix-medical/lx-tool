//! # Install Module
//! Installs the Helix stack
//!
//! Used only when the administrator create an Helix Instance.
//!
//! # Run
//! ```bash
//! lx install
//! ```
//! # Output
//! ```text
//! --     Installing Helix     --
//! ... (omitted)
//! ```
use super::docker;
use local_ip_address::local_ip;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use std::io::{Result, Stdin};

// Docker Compose file for dev environment
// const DOCKERCOMPOSE_DEV: &str =
//     "-f/Users/xavier2p/Developer/javascript/helix/docker-compose.dev.yml";

/// Docker Compose file for prod environment
const DOCKERCOMPOSE_PROD: &str = "-f./docker-compose.yml";

/// Installs the Helix stack
/// # Example
/// ```
/// use lx::install::all;
/// all();
/// ```
/// # Output
/// ```text
/// --     Installing Helix     --
/// ... (omitted)
/// ```
pub fn all() {
    println!(" --    Installing Helix     --\n");
    create_dir().expect("Could not create config directory");
    let stdin: Stdin = std::io::stdin();
    let db_password: String = get_db_password(stdin).expect("Could not get database password");
    let ip_address: String = get_ip_address().expect("Could not get IP address");
    create_random_token("access-token").expect("Could not create access token");
    create_random_token("refresh-token").expect("Could not create refresh token");
    println!("Pulling the latest images...");
    docker::compose(DOCKERCOMPOSE_PROD, &["pull"]);
    sum_up(db_password, ip_address);
}

/// Creates the config directory
/// # Example
/// ```
/// use lx::install::create_dir;
/// create_dir();
/// ```
/// # Output
/// ```text
/// Creating config directory...
/// ```
/// # Errors
/// ... !TODO
fn create_dir() -> Result<()> {
    println!("Creating config directory...");
    fs::create_dir("./config")?;
    Ok(())
}

/// Gets the IP address of the machine and stores it in a file
/// # Example
/// ```
/// use lx::install::get_ip_address;
/// get_ip_address();
/// ```
/// # Output
/// ```text
/// IP Address: {Your IP Address}
/// ```
/// # Errors
/// ... !TODO
fn get_ip_address() -> Result<String> {
    let ip_address: String = local_ip().expect("Could not get IP address").to_string();
    println!("IP Address: {}", ip_address);
    fs::write("./config/ip-address", ip_address.clone())?;
    fs::write(
        "./config/ip-address-api",
        format!("http://{}:3001/api", ip_address.clone()),
    )?;
    Ok(ip_address)
}

/// Gets the database password from the user and stores it in a file
/// # Arguments
/// * `stdin` - The standard input stream
/// # Example
/// ```
/// use lx::install::get_db_password;
/// let stdin = std::io::Stdin();
/// get_db_password(stdin);
/// ```
/// # Output
/// ```text
/// Choose database password: (default: root)
/// ... (enter password)
/// ```
/// # Errors
/// ... !TODO
fn get_db_password(stdin: Stdin) -> Result<String> {
    println!("Choose database password: (default: root)");
    let mut password: String = String::from("root");
    stdin.read_line(&mut password)?;
    fs::write("./config/db-password", password.clone())?;
    Ok(password)
}

/// Creates a random token and store it in a file
/// # Arguments
/// * `name` - The name of the token
/// # Example
/// ```
/// use lx::install::create_random_token;
/// create_random_token("access-token");
/// ```
/// # Output
/// ```text
/// Generating access-token...
/// ```
/// # Errors
/// ... !TODO
fn create_random_token(name: &str) -> Result<String> {
    println!("Generating {}...", name);
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    fs::write(format!("./config/{}", name), token.clone())?;
    Ok(token)
}

/// Summarizes the installation
/// # Arguments
/// * `password` - The database password
/// * `ip_address` - The IP address of the machine
/// # Example
/// ```
/// use lx::install::sum_up;
/// sum_up("root".to_string(), "127.0.0.1".to_string());
/// ```
/// # Output
/// ```text
/// --     Helix Installed     --
/// Database password: root
/// IP Address: 127.0.0.1
/// Ready to start Helix!
/// ```
/// # Errors
/// ... !TODO
fn sum_up(password: String, ip_address: String) {
    println!(" --     Helix Installed     --\n");
    println!("Database password: {}", password);
    println!("IP Address: {}", ip_address);
    println!("Ready to start Helix!")
}
