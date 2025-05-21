use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::mpsc::channel,
};
use tokio::time::{sleep, Duration};

#[derive(Parser)]
#[command(name = "logtail")]
#[command(about = "Tail multiple log files in a directory", long_about = None)]
struct Cli {
    #[arg(help = "Directory containing log files to watch")]
    directory: String,
}

struct LogFile {
    reader: BufReader<File>,
    name: String,
}

impl LogFile {
    fn new(path: &Path) -> Result<Self> {
        let file = File::open(path).context("Failed to open log file")?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::End(0))?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        Ok(LogFile { reader, name })
    }

    fn read_new_content(&mut self) -> Result<String> {
        let mut buffer = String::new();
        self.reader.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

async fn watch_directory(directory: &str) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(Path::new(directory).as_ref(), RecursiveMode::Recursive)?;

    let mut log_files: HashMap<PathBuf, LogFile> = HashMap::new();

    // Initial scan for .log files
    for entry in std::fs::read_dir(directory)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()).map_or(false, |ext| ext.to_lowercase() == "log") {
            let log_file = LogFile::new(&path)?;
            log_files.insert(path, log_file);
        }
    }

    println!("Watching for changes in {}...", directory.bright_blue());
    println!("Press Ctrl+C to stop");

    loop {
        match rx.recv() {
            Ok(event) => {
                handle_event(event, &mut log_files).await?;
            }
            Err(e) => println!("Watch error: {}", e),
        }
        // Small delay to prevent CPU spinning
        sleep(Duration::from_millis(100)).await;
    }
}

async fn handle_event(
    event: Result<Event, notify::Error>,
    log_files: &mut HashMap<PathBuf, LogFile>,
) -> Result<()> {
    match event {
        Ok(event) => {
            for path in event.paths {
                if path.extension().and_then(|s| s.to_str()).map_or(false, |ext| ext.to_lowercase() == "log") {
                    match event.kind {
                        notify::EventKind::Create(_) => {
                            match LogFile::new(&path) {
                                Ok(log_file) => {
                                    log_files.insert(path.clone(), log_file);
                                    println!(
                                        "{} New log file detected: {}",
                                        "➕".green(),
                                        path.display()
                                    );
                                }
                                Err(e) => {
                                    eprintln!("{} Failed to watch new log file {}: {}", "⚠️".red(), path.display(), e);
                                }
                            }
                        }
                        notify::EventKind::Modify(_) => {
                            if let Some(log_file) = log_files.get_mut(&path) {
                                match log_file.read_new_content() {
                                    Ok(content) => {
                                        if !content.is_empty() {
                                            // Print a newline to ensure this block starts on a new line
                                            println!();
                                        // Print the file name indicator
                                        println!(
                                            "{} {}:",
                                            "📝".yellow(),
                                            log_file.name.bright_green()
                                        );
                                        // Print the content
                                        for line in content.lines() {
                                            println!("  {}", line);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{} Error reading from log file {}: {}", "⚠️".red(), log_file.name.bright_red(), e);
                                }
                            }
                        }
                        notify::EventKind::Remove(_) => {
                            if log_files.remove(&path).is_some() {
                                println!(
                                    "{} Removed log file: {}",
                                    "➖".red(),
                                    path.display()
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    watch_directory(&cli.directory).await?;
    Ok(())
}
