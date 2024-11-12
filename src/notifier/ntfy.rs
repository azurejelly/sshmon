use crate::notifier::Notifier;

pub struct NtfyNotifier {}

impl Notifier for NtfyNotifier {
    fn send_silent_notif(&self, _source_ip: &str, _user: &str, _method: &str) {
        // println!("inc: from {} using user {} ({})", source_ip, user, method);
    }
}