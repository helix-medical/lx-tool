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
use std::{
    fs,
    io::{prelude::*, stdin, stdout, Stdin},
};

// Docker Compose file for dev environment
// const DOCKERCOMPOSE_DEV: &str =
//     "-f/Users/xavier2p/Developer/javascript/helix/docker-compose.dev.yml";

/// Error type for the install module
/// # Variants
/// * `Io` - An error from the `std::io` module
/// * `LocalIp` - An error from the `local_ip_address` module
/// # Example
/// ```
/// use lx::install::InstallError;
/// let error = InstallError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Error"));
/// ```
#[derive(Debug)]
enum InstallError {
    Io(std::io::Error),
    LocalIp(local_ip_address::Error),
}

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
    create_dotenv().expect("Could not create config directory");
    let db_password: String = get_db_password(stdin()).expect("Could not get database password");
    let ip_address: String = get_ip_address().expect("Could not get IP address");
    create_random_token("ACCESS_TOKEN").expect("Could not create access token");
    create_random_token("REFRESH_TOKEN").expect("Could not create refresh token");
    println!("Pulling the latest images...");
    docker::compose(DOCKERCOMPOSE_PROD, &["pull"]);
    sum_up(db_password, ip_address);
}

/// Creates the `.env§ file
/// # Example
/// ```
/// use lx::install::create_dir;
/// create_dotenv();
/// ```
/// # Output
/// ```text
/// Creating `.env` file...
/// ```
fn create_dotenv() -> std::io::Result<()> {
    println!("Creating `.env` file...");
    fs::write(".env", "")
}

/// Appends a line to the `.env` file
/// # Arguments
/// * `content` - The content to append
/// # Returns
/// * `Result<(), InstallError>` - The result of the operation
///
/// # Example
/// ```
/// use lx::install::append_to_dotenv;
/// append_to_dotenv("IP_ADDRESS=127.0.0.1".to_string());
/// ```
fn append_to_dotenv(content: String) -> Result<(), InstallError> {
    let mut env_file = fs::OpenOptions::new()
        .append(true)
        .open(".env")
        .map_err(|e| InstallError::Io(e))?;

    writeln!(env_file, "{}", content).map_err(|e| InstallError::Io(e))?;

    Ok(())
}

/// Gets the IP address of the machine and stores it in the `.env` file
/// # Example
/// ```
/// use lx::install::get_ip_address;
/// get_ip_address();
/// ```
/// # Output
/// ```text
/// IP Address: 127.0.0.1
/// ```
fn get_ip_address() -> Result<String, InstallError> {
    let ip_address = local_ip().map_err(|e| InstallError::LocalIp(e))?;
    let api_ip_address: String = format!("API_URL=http://{}:3001/api", ip_address.clone());
    println!("{}", api_ip_address);
    let ip_address: String = format!("IP_ADDRESS={}", ip_address);
    println!("{}", ip_address);

    append_to_dotenv(api_ip_address)?;
    append_to_dotenv(ip_address.clone())?;
    Ok(ip_address)
}

/// Gets the database password from the user and stores it in the `.env` file
/// # Arguments
/// * `stdin` - The standard input stream
/// # Example
/// ```
/// use lx::install::get_db_password;
/// let stdin = std::io::stdin();
/// get_db_password(stdin);
/// ```
/// # Output
/// ```text
/// Choose database password: (default: root)
/// ... (enter password)
/// ```
fn get_db_password(stdin: Stdin) -> Result<String, InstallError> {
    print!("Choose database password: (default: root) ");
    stdout().flush().map_err(|e| InstallError::Io(e))?;
    let mut password: String = String::from("");
    stdin
        .read_line(&mut password)
        .map_err(|e| InstallError::Io(e))?;

    if password.trim() == "" {
        password = String::from("root");
    }

    let password_file: String = format!("DATABASE_PASSWORD={}", password.trim());
    append_to_dotenv(password_file)?;

    Ok(password.trim().to_string())
}

/// Creates a random token and store it in the `.env` file
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
fn create_random_token(name: &str) -> Result<String, InstallError> {
    println!("Generating {}...", name);
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    append_to_dotenv(format!("{}={}", name, token.clone()))?; // TODO: Handle error
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
fn sum_up(password: String, ip_address: String) {
    println!(" --     Helix Installed     --\n");
    println!("Database password: {}", password);
    println!("IP Address: {}", ip_address);
    println!("Ready to start Helix!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_create_dotenv() {
        // Test that the function returns Ok(())
        assert!(create_dotenv().is_ok());

        // Test that the .env file was created
        assert!(Path::new(".env").exists());

        // Test that the .env file is empty
        let contents = fs::read_to_string(".env").unwrap();
        assert_eq!(contents, "");

        // Test if the function does nothing if the file already exists
        assert!(create_dotenv().is_ok());

        // Cleanup
        fs::remove_file(".env").expect("Could not delete .env file");
        assert!(Path::new(".env").exists() == false);
    }

    #[test]
    #[serial]
    fn test_append_to_dotenv() {
        // No file, must return an error
        assert!(append_to_dotenv("TEST_KEY=TEST_VALUE".to_string()).is_err());

        // Create the .env file
        create_dotenv().expect("Could not create .env file");

        // Test that the function returns Ok(())
        let result = append_to_dotenv("TEST_KEY=TEST_VALUE".to_string());
        assert!(result.is_ok());

        // Test that the .env file was updated
        let contents = fs::read_to_string(".env").unwrap();
        assert!(contents.contains("TEST_KEY=TEST_VALUE"));
    }

    #[test]
    #[serial]
    fn test_get_ip_address() {
        // Create the .env file
        create_dotenv().expect("Could not create .env file");

        // Test that the function returns a valid IP address
        let result = get_ip_address();
        assert!(result.is_ok());

        // Test that the IP address is correct
        let ip_address = result.unwrap();
        assert!(ip_address.starts_with("IP_ADDRESS="));
        assert!(ip_address.contains(".")); // Check that the IP address contains a dot

        // Test if the file is correctly written
        let contents = fs::read_to_string(".env").unwrap();
        assert!(contents.contains("IP_ADDRESS="));
        assert!(contents.contains("."));
        assert!(contents.contains("API_URL="));

        // Cleanup
        fs::remove_file(".env").expect("Could not delete .env file");
        assert!(Path::new(".env").exists() == false);
    }

    #[test]
    #[serial]
    fn test_create_random_token() {
        // Create the .env file
        create_dotenv().expect("Could not create .env file");

        // Test that the function returns a token of the correct length
        let result = create_random_token("test_token");
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.len(), 128);

        // Test that the function appends the token to the .env file
        let contents = fs::read_to_string(".env").unwrap();
        assert!(contents.contains("test_token="));
    }
}
