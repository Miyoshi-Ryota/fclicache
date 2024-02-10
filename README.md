![Unit Test and Lint Status](https://github.com/Miyoshi-Ryota/fclicache/actions/workflows/ci.yml/badge.svg)
[![Docs.rs](https://docs.rs/fclicache/badge.svg)](https://docs.rs/fclicache/latest/fclicache/)
[![Crates.io](https://img.shields.io/crates/v/fclicache.svg)](https://crates.io/crates/fclicache)

# fclicache - File-based Simple CLI Cache

fclicache is a command-line utility that caches the output of resource-intensive commands, enabling faster retrieval of results when the same commands are executed repeatedly within a specified Time-to-Live (TTL) period. This tool is designed to enhance efficiency and save time, particularly in development environments where certain commands are run frequently.

# Features

- **Efficient Caching:** Saves the output of CLI commands, avoiding repeated execution of time-consuming operations.
- **Time-to-Live (TTL):** Customizable cache duration to ensure the freshness of the cached data.
- **Simple Usage:** Easy to integrate into existing workflows with minimal configuration.

# Installation

Before installing fclicache, ensure you have Rust and Cargo installed on your system. If not, follow the installation guide here: https://www.rust-lang.org/tools/install.

To install fclicache, run the following command:

```bash
cargo install fclicache
```

# Usage

To cache the output of a command using fclicache, wrap the desired command as follows:

```bash
fclicache --ttl [SECONDS] '[COMMAND]'
```

The tool caches the output of [COMMAND] for the duration specified by [SECONDS].

## Example

```bash
# First execution: the 'sleep 10 && date' command will take approximately 10 seconds.
$ fclicache --ttl 3600 'sleep 10 && date'
Sat Feb 10 19:07:49 JST 2024

# Second execution: results are instantly retrieved from cache, showing the same output as the first execution.
$ fclicache --ttl 3600 'sleep 10 && date'
Sat Feb 10 19:07:49 JST 2024
```

## Note
fclicache utilizes a simple file-based caching mechanism. This means that your command's output is written to a file. Therefore, it is advised not to use this tool for commands that produce sensitive or secure output.

# Contributing
We are very open to any contributions! Your input is invaluable to us, and we strive to make incorporating your suggestions and improvements as seamless as possible.

- **Open to All:** Whether it's feature suggestions, bug reports, or code improvements, every contribution is welcome.
- **Ongoing Maintenance:** Even if this repository might appear inactive or outdated at times, rest assured that we actively review and accept pull requests.

## Show Your Support

If you find `fclicache` useful, consider giving it a star on GitHub! Your support motivates us to continuously improve and maintain the project.

[Star `fclicache` on GitHub](https://github.com/Miyoshi-Ryota/fclicache)
