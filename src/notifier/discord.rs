use std::time::SystemTime;

use chrono::{DateTime, Utc};
use log::{error, info};
use serde_json::json;

use crate::notifier::Notifier;

pub struct DiscordNotifier {
    hostname: String,
    url: String,
}

impl DiscordNotifier {
    pub fn new(hostname: String, url: String) -> Self {
        Self { hostname, url }
    }
}

impl Notifier for DiscordNotifier {
    fn send_silent_notif(&self, source_ip: &str, user: &str, method: &str) {
        let time = SystemTime::now();
        let into: DateTime<Utc> = time.into();
        let iso8601 = into.to_rfc3339();
        
        let json = json!(
            {
                "content": null,
                "embeds": [
                  {
                    "title": format!("SSH login at {}", self.hostname),
                    "description": format!("User `{}` has logged in from `{}` using `{}` authentication.", user, source_ip, method),
                    "color": 15564017,
                    "timestamp": iso8601,
                  }
                ]
            }
        );

        let client = reqwest::blocking::Client::new();
        let req = client.post(&self.url)
            .header("Content-Type", "application/json")
            .body(json.to_string())
            .send();

        if let Err(e) = &req {
            error!("Failed to send Discord webhook message: {:?}", e);
        }

        let res = &req.unwrap();
        info!("{:?}", res);
    }
}
