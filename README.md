# logtail

A Rust-based tool for monitoring multiple log files in a directory simultaneously.

## Features

- Monitor all `.log` files in a specified directory
- Real-time updates with colorized output
- Automatic detection of new log files
- Notification when log files are removed
- Efficient async I/O operations

## Installation

```bash
cargo install --path .
```

## Usage

```bash
logtail <directory_path>
```

Example:
```bash
logtail /var/log
```

## Output Format

The tool uses color-coded icons to indicate different events:
- 📝 (Yellow) - File modifications
- ➕ (Green) - New log files detected
- ➖ (Red) - Log files removed

## Building from Source

```bash
cargo build --release
```

The binary will be available at `target/release/logtail`

## Dependencies

- clap: Command line argument parsing
- notify: File system notifications
- tokio: Async runtime
- anyhow: Error handling
- colored: Terminal colors
