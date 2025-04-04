use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};

use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Use: log-archive <logs-directory>");
        process::exit(1);
    }

    let log_dir = Path::new(&args[1]);
    if !log_dir.exists() || !log_dir.is_dir() {
        eprintln!("Error: '{}' is not a valid directory.", log_dir.display());
        process::exit(1);
    }

    let archive_dir = Path::new("archived_logs");
    if let Err(e) = fs::create_dir_all(&archive_dir) {
        eprintln!("Error creating directory: {}", e);
        process::exit(1);
    }

    match create_archive(log_dir, archive_dir) {
        Ok(archive_path) => {
            if let Err(e) = log_event(&archive_path, archive_dir) {
                eprintln!("Error registering event: {}", e);
            } else {
                println!("✔️ File successfully created: {}", archive_path.display());
            }
        }
        Err(e) => {
            eprintln!("Error creating the file: {}", e);
            process::exit(1);
        }
    }
}

fn create_archive(log_dir: &Path, archive_dir: &Path) -> io::Result<PathBuf> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let archive_name = format!("logs_archive_{}.tar.gz", timestamp);
    let archive_path = archive_dir.join(archive_name);

    let tar_gz = File::create(&archive_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_dir_all(log_dir.file_name().unwrap(), log_dir)?;

    Ok(archive_path)
}

fn log_event(archive_path: &Path, archive_dir: &Path) -> io::Result<()> {
    let log_file = archive_dir.join("archive_log.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    writeln!(
        file,
        "[{}] File created: {}",
        timestamp,
        archive_path.file_name().unwrap().to_string_lossy()
    )?;

    Ok(())
}
