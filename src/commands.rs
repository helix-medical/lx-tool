use std::process::Command;

use super::docker;

/// Docker Compose file for dev environment
const DOCKERCOMPOSE_DEV: &str =
    "-f/Users/xavier2p/Developer/javascript/helix/docker-compose.dev.yml";

/// Docker Compose file for prod environment
const DOCKERCOMPOSE_PROD: &str =
    "-f/Users/xavier2p/Developer/javascript/helix/releases/docker-compose.yml";

const DOT_ENV_CONFIG: &str = ".env";

/// Starts the Helix stack
/// # Arguments
/// * `is_dev` - Whether to start the dev or prod stack
/// # Example
/// ```
/// use tool::commands::start;
/// start(true);
/// ```
pub fn start(is_dev: bool) {
    println!(" --     Starting Helix     --\n");
    Command::new("source")
        .arg(DOT_ENV_CONFIG)
        .spawn()
        .expect("Could not source .env file");
    if is_dev {
        docker::compose(DOCKERCOMPOSE_DEV, &["up", "--build"]);
    } else {
        docker::compose(DOCKERCOMPOSE_PROD, &["up", "-d"]);
    }
}

/// Stops the Helix stack
/// # Arguments
/// * `is_dev` - Whether to stop the dev or prod stack
/// # Example
/// ```
/// use tool::commands::stop;
/// stop(true);
/// ```
pub fn stop(is_dev: bool) {
    println!(" --     Stopping Helix     --\n");
    docker::compose(
        if is_dev {
            DOCKERCOMPOSE_DEV
        } else {
            DOCKERCOMPOSE_PROD
        },
        &["down"],
    );
}

/// Cleans the Helix stack
/// # Arguments
/// * `is_dev` - Whether to clean the dev or prod stack
/// # Example
/// ```
/// use tool::commands::clean;
/// clean(true);
/// ```
pub fn clean(is_dev: bool) {
    println!(" --     Cleaning Helix     --\n");
    let images = if is_dev {
        vec!["helix-dev-server", "helix-dev-db", "helix-dev-client"]
    } else {
        vec![
            "xavier2p/helix-server",
            "xavier2p/helix-db",
            "xavier2p/helix-client",
        ]
    };
    for image in images {
        docker::rmi(image);
    }
}
