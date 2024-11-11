use pushover::PushoverNotifier;
use stdout::StdoutNotifier;

use crate::config::Config;

pub mod ntfy;
pub mod pushover;
pub mod stdout;

pub trait Notifier {
    fn send_silent_notif(&self, source_ip: &str, user: &str, method: &str);
}

pub enum NotifierType {
    Stdout,
    Pushover,
    // Ntfy
}

impl std::str::FromStr for NotifierType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "stdout" => Ok(NotifierType::Stdout),
            "pushover" => Ok(NotifierType::Pushover),
            // "ntfy" => Ok(NotifierType::Ntfy),
            _ => Err(format!("unknown notifier type {s}"))
        }
    }
}

pub fn get_notifier(cfg: &Config) -> Box<dyn Notifier> {
    match &cfg.notifier {
        NotifierType::Stdout => Box::new(StdoutNotifier::new()),
        NotifierType::Pushover => {
            let host = cfg.hostname_override.clone().unwrap_or_else(|| {
                gethostname::gethostname()
                    .into_string()
                    .unwrap_or_else(|_| {
                        panic!("could not obtain a valid hostname, please set the HOSTNAME_OVERRIDE environment variable instead")
                    })
            });

            let api = cfg.pushover_api_key.clone()
                .expect("a pushover API key must be specified");

            let usr = cfg.pushover_user_key.clone()
                .expect("a pushover user key must be specified");

            Box::new(PushoverNotifier::new(host.to_string(), api.to_string(), usr.to_string()))
        }
    }
}