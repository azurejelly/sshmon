use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::mpsc;
use log::{debug, error};
use notify::event::ModifyKind;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use regex::Regex;
use std::thread;
use std::time::Duration;
use dotenv::dotenv;

pub mod config;
pub mod notifier;
pub mod logger;

fn main() -> Result<()> {
    dotenv().ok();

    let config = config::get();
    logger::init(&config).expect("Failed to initialize logger");

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
                
                // is there a way to read the data directly from notify
                // please tell me if there is
                if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
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

        debug!("Sleeping for 100 ms");
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
