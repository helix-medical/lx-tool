# `lx` | The Helix Utility Tool

The Helix Utility Tool is a command line tool that allows you to manage the Helix Server.

## Usage

```bash
lx [FLAGS] <COMMAND>
```

## Flags

* `-h`, `--help` - Prints help information
* `-V`, `--version` - Prints version information
* `--dev` - Sets dev mode

## Commands

* `start` - Starts the Helix Server
* `stop` - Stops the Helix Server
* `restart` - Restarts the Helix Server
* `status` - Prints the status of the Helix Server
* `logs` - Prints the logs of the Helix Server
* `clean` - Cleans the Helix Server

## Examples

```bash
lx start
lx stop
```

## Installation

```bash
git clone <REPO_URL> && cd lx-tool
cargo build --release
```

## Add to PATH

```bash
sudo cp target/release/lx /usr/local/bin
```
