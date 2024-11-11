pub mod ntfy;
pub mod pushover;
pub mod stdout;

pub trait Notifier {
    fn send_silent_notif(source_ip: &str, user: &str, method: &str);
}