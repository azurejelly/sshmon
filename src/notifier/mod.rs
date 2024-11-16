use discord::DiscordNotifier;
use pushover::PushoverNotifier;
use stdout::StdoutNotifier;

use crate::config::Config;

pub mod ntfy;
pub mod pushover;
pub mod stdout;
pub mod discord;

pub trait Notifier {
    fn send_silent_notif(&self, source_ip: &str, user: &str, method: &str);
}

pub enum NotifierType {
    Stdout,
    Pushover,
    Discord,
    // Ntfy
}

impl std::str::FromStr for NotifierType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "stdout" => Ok(NotifierType::Stdout),
            "pushover" => Ok(NotifierType::Pushover),
            "discord" => Ok(NotifierType::Discord),
            // "ntfy" => Ok(NotifierType::Ntfy),
            _ => Err(format!("Unknown notifier type {}", s))
        }
    }
}

pub fn get_notifier(cfg: &Config) -> Box<dyn Notifier> {
    match &cfg.notifier {
        NotifierType::Stdout => Box::new(StdoutNotifier::new()),
        NotifierType::Discord => {
            let host = get_hostname(&cfg);

            let url = cfg.discord_webhook
                .clone()
                .expect("DISCORD_WEBHOOK must be set");
            
            Box::new(DiscordNotifier::new(host, url))
        },
        NotifierType::Pushover => {
            let host = get_hostname(&cfg);

            let api = cfg.pushover_api_key
                .clone()
                .expect("A Pushover API key must be specified");

            let usr = cfg.pushover_user_key
                .clone()
                .expect("A Pushover user key must be specified");

            Box::new(PushoverNotifier::new(host, api, usr))
        }
    }
}

fn get_hostname(cfg: &Config) -> String {
    cfg.hostname_override.clone().unwrap_or_else(|| {
        gethostname::gethostname()
            .into_string()
            .unwrap_or_else(|_| {
                panic!("Could not obtain a valid hostname, please set the HOSTNAME_OVERRIDE environment variable instead")
            })
    })
}