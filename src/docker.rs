use std::process::Command;

/// Runs a docker-compose command
/// # Arguments
/// * `path` - The path to the docker-compose file
/// * `args` - The arguments to pass to docker-compose
/// # Example
/// ```
/// use tool::docker::compose;
/// compose("docker-compose.yml", &["up", "-d"]);
/// ```
pub fn compose(path: &str, args: &[&str]) {
    Command::new("docker-compose")
        .arg(path)
        .args(args)
        .status()
        .expect("failed to execute process");
}

/// # Description
/// Removes a Docker Image
/// ## Arguments
/// * `image` - The name of the image to remove
/// ## Example
/// ```
/// use tool::docker::rmi;
/// rmi("xavier2p/helix-server");
/// ```
pub fn rmi(image: &str) {
    Command::new("docker")
        .arg("image")
        .arg("rm")
        .arg(image)
        .status()
        .expect("docker command failed to start");
}
