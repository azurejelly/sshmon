use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc;
use log::{debug, error};
use notify::event::{ModifyKind, RenameMode};
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use regex::Regex;
use dotenv::dotenv;

pub mod config;
pub mod notifier;

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::get();
    let re = Regex::new(&config.auth_sucess_regex).unwrap();
    let notif = notifier::get_notifier(&config);
    
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&config.ssh_logs_path), RecursiveMode::Recursive)?;

    let mut file = File::open(&config.ssh_logs_path).expect("Failed to open log file");
    file.seek(SeekFrom::End(0)).unwrap();

    let mut last_position = file.stream_position().unwrap();
    for res in rx {
        match res {
            Ok(event) => {
                debug!("Received event {:?} on file {}", event, config.ssh_logs_path);
                debug!("Last known position is {}", last_position);
                
                // If the file gets created again, reset the last position to zero.
                // A possible cause for this is, for example, log rotation.
                if event.kind.is_create() {
                    debug!("File was created again, resetting last position and reopening file.");

                    file = File::open(&config.ssh_logs_path)?;
                    file.seek(SeekFrom::End(0)).unwrap();
                    last_position = file.stream_position().unwrap();
                    continue;
                }

                // We also want to reset the last position to zero when the file is renamed.
                if let EventKind::Modify(ModifyKind::Name(rename_mode)) = event.kind {
                    debug!("Detected a file rename, resetting last position and reopening file: {:?}", rename_mode);

                    file = File::open(&config.ssh_logs_path)?;
                    file.seek(SeekFrom::End(0))?;
                    last_position = file.stream_position()?;
                    continue;
                }

                // Please tell me if there's a way to read stuff from the event directly
                // because I honestly have no idea if it is possible
                if event.kind.is_modify() {
                    file.seek(SeekFrom::Start(last_position)).unwrap();

                    let reader = BufReader::new(&file);
                    for line in reader.lines() {
                        if let Ok(log) = line {
                            if let Some(caps) = re.captures(&log) {
                                let mtd: &str = &caps[1];
                                let usr = &caps[2];
                                let src = &caps[3];
                                
                                notif.send_silent_notif(src, usr, mtd);
                            }
                        }
                    }

                    last_position = file.stream_position().unwrap();
                }
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
