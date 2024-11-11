use crate::notifier::Notifier;

pub struct StdoutNotifier {}

impl Notifier for StdoutNotifier {
    fn send_silent_notif(source_ip: &str, user: &str, method: &str) {
        println!("inc: from {} using user {} ({})", source_ip, user, method);
    }
}