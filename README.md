# logtail

A Rust-based tool for monitoring multiple log files in a directory simultaneously.

## Features

- Monitor all `.log` files (case-insensitive, e.g., `my.log`, `MY.LOG`, `my.LoG`) in a specified directory
- Real-time updates with colorized output
- Automatic detection of new log files
- Notification when log files are removed
- Efficient async I/O operations

## Behavior

- **Initial Content**: `logtail` is designed to show new content appended to log files *after* it has started. This also applies to newly created files; only content written after detection will be displayed. It does not display the pre-existing content of files upon startup.
- **Error Reporting**: The tool will report errors related to accessing or reading individual log files to `stderr` (e.g., if a file is deleted or permissions change) but will continue to monitor other files.

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
- 📝 (Yellow) - File modifications (new content)
- ➕ (Green) - New log files detected
- ➖ (Red) - Log files removed
- ⚠️ (Red) - Errors reading a specific log file (details printed to stderr)

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
