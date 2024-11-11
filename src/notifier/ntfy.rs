use crate::notifier::Notifier;

pub struct NtfyNotifier {}

impl Notifier for NtfyNotifier {
    fn send_silent_notif(&self, source_ip: &str, user: &str, method: &str) {
        // println!("inc: from {} using user {} ({})", source_ip, user, method);
    }
}