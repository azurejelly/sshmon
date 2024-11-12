use ntfy::{Dispatcher, Payload};

use crate::notifier::Notifier;

pub struct NtfyNotifier {
    hostname: String,
    url: String,
    username: String,
    password: String,
    topic: String,
}

impl NtfyNotifier {
    fn new(hostname: String, url: String, username: String, password: String, topic: String) -> Self {
        Self { hostname, url, username, password, topic }
    }
}

impl Notifier for NtfyNotifier {
    fn send_silent_notif(&self, source_ip: &str, user: &str, method: &str) {
        let dispatcher = Dispatcher::builder(&self.url)
            .credentials(Auth::credentials(&self.username, &self.password))
            .build()?;

        let payload = Payload::new(self.topic)
            .message(format!("{} has logged in from {} via {}", user, source_ip, method))
            .title(format!("New SSH login at {}", self.hostname))
            .priority(ntfy::Priority::Low);

        dispatcher.send(&payload)?;
    }
}