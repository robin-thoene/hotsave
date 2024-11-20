use clap::Parser;
use config::{Config, File};
use env_logger::Env;
use log::{debug, error, info};
use rdev::{grab, Event, EventType, Key};
use serde::{self, Deserialize};
use std::{env, fmt::Debug, fs::copy, path::PathBuf, process::exit};

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

#[derive(Deserialize, Debug)]
struct AppConfig {
    save_file_key: Key,
    restore_file_key: Key,
}

fn main() {
    // Setup
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Parse possibly existing config
    let home = env::var("HOME");
    let mut config_builder = Config::builder()
        .set_default("save_file_key", "F4")
        .expect("The default save file key must be set")
        .set_default("restore_file_key", "F5")
        .expect("The default restore file key must be set");
    if let Ok(home) = home {
        config_builder = config_builder.add_source(
            File::with_name(&format!("{}/.config/hotsave/hotsave.toml", home)).required(false),
        );
    }
    let config = config_builder
        .build()
        .expect("Expect a configuration, at least initialized with defaults")
        .try_deserialize::<AppConfig>()
        .expect("At least the defaul config should be deserializable");
    debug!("Loaded config: {:#?}", config);
    // Check cli arguments
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
    info!(
        "Key to save current file version: {:#?}",
        config.save_file_key
    );
    info!(
        "Key to restore saved file version: {:#?}",
        config.restore_file_key
    );
    // Build the callback for the key listener
    let grab_callback = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyPress(k) if k == config.save_file_key => {
                info!("Saving current file version");
                _ = copy(&file_path, &save_file_path);
                None
            }
            EventType::KeyPress(k) if k == config.restore_file_key => {
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
