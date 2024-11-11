use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use notifier::stdout::StdoutNotifier;
use notifier::Notifier;
use regex::Regex;
use std::thread;
use std::time::Duration;

pub mod config;
pub mod notifier;

fn main() {
    let config = config::init();
    let re = Regex::new(&config.auth_sucess_regex).unwrap();
    
    let mut file = File::open(config.ssh_logs_path).expect("Unable to open log file");
    file.seek(SeekFrom::End(0)).unwrap();

    loop {
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            if let Ok(log) = line {
                if let Some(caps) = re.captures(&log) {
                    let method = &caps[1];
                    let user = &caps[2];
                    let source_ip = &caps[3];

                    println!("Found valid SSH login, sending silent notification");
                    StdoutNotifier::send_silent_notif(source_ip, user, method);
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
        println!("Sleeping main thread for 1000 ms");
        file.seek(SeekFrom::End(0)).unwrap();
    }
}
