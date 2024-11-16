use log::{debug, error};
use pushover::{requests::message::SendMessage, API};

use crate::notifier::Notifier;

pub struct PushoverNotifier {
    pub hostname: String,
    pub api_key: String,
    pub user_key: String,
}

impl PushoverNotifier {
    pub fn new(hostname: String, api_key: String, user_key: String) -> Self {
        Self { hostname, api_key, user_key }
    }
}

impl Notifier for PushoverNotifier {
    fn send_silent_notif(&self, source_ip: &str, user: &str, method: &str) {
        debug!("Called PushoverNotifier#send_silent_notif(...)");
        let api = API::new();
        let mut msg = SendMessage::new(
            self.api_key.to_string(), 
            self.user_key.to_string(), 
            format!("{} has logged in from {} via {}", user, source_ip, method)
        );
        
        msg.set_priority(pushover::Priority::Low);
        msg.set_title(format!("New SSH login at {}", self.hostname));

        debug!("Finished building message, sending through Pushover API");
        let res = api.send(&msg);
        match res {
            Ok(_) => debug!("Pushover request was successful"),
            Err(e) => error!("Failed to send Pushover message: {}", e),
        }
    }
}