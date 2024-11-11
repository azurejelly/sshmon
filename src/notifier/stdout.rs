use crate::notifier::Notifier;

pub struct StdoutNotifier {}

impl StdoutNotifier {
    pub fn new() -> Self {
        StdoutNotifier {}
    }
}

impl Notifier for StdoutNotifier {
    fn send_silent_notif(&self, source_ip: &str, user: &str, method: &str) {
        println!("Logged in from {} using user {} via {}", source_ip, user, method);
    }
}