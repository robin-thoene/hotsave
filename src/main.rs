use clap::Parser;
use env_logger::Env;
use log::{debug, error, info};
use rdev::{grab, Event, EventType, Key};
use std::{fmt::Debug, fs::copy, path::PathBuf, process::exit};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "The path to the file to save/restore using the global hotkeys"
    )]
    path: PathBuf,
}

fn main() {
    // Setup
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    debug!(
        "File path \"{}\" was provided as argument",
        args.path.display()
    );
    // Validate arguments
    if !args.path.exists() || !args.path.is_file() {
        error!("Nothing exists on provided path, or it is no file");
        exit(1);
    }
    let file_name = args
        .path
        .file_stem()
        .expect("The file name is required to proceed");
    debug!("File name is {}", file_name.to_string_lossy());
    let file_extension = args
        .path
        .extension()
        .expect("The file extension is required to proceed");
    debug!("File extension is {}", file_extension.to_string_lossy());
    // Build required variables
    let file_path = args.path.clone();
    let save_file_path = args.path.with_file_name(format!(
        "{}_tmp_save.{}",
        file_name.to_string_lossy(),
        file_extension.to_string_lossy()
    ));
    let save_file_key = Key::F4;
    let restore_file_key = Key::F5;
    info!("Key to save current file version: {:#?}", save_file_key);
    info!("Key to restore saved file version: {:#?}", restore_file_key);
    // Build the callback for the key listener
    let grab_callback = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyPress(k) if k == save_file_key => {
                info!("Saving current file version");
                _ = copy(&file_path, &save_file_path);
                None
            }
            EventType::KeyPress(k) if k == restore_file_key => {
                info!("Restore saved file version");
                _ = copy(&save_file_path, &file_path);
                None
            }
            _ => {
                debug!("Detected {:#?}, but ignored it", event);
                Some(event)
            }
        }
    };
    // Listen to key inputs globally
    if let Err(error) = grab(grab_callback) {
        error!("Error: {:?}", error)
    }
}
