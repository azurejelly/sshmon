use envconfig::Envconfig;

use log::debug;

use crate::notifier::NotifierType;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "SSH_LOGS_PATH", default = "/var/log/auth.log")]
    pub ssh_logs_path: String,
    
    #[envconfig(from = "AUTH_SUCCESS_REGEX", default = r"Accepted (\w+) for (\w+) from ([\d\.]+) port ")]
    pub auth_sucess_regex: String,

    #[envconfig(from = "NOTIFIER", default = "pushover")]
    pub notifier: NotifierType,

    #[envconfig(from = "PUSHOVER_API_KEY")]
    pub pushover_api_key: Option<String>,

    #[envconfig(from = "PUSHOVER_USER_KEY")]
    pub pushover_user_key: Option<String>,

    #[envconfig(from = "HOSTNAME_OVERRIDE")]
    pub hostname_override: Option<String>,

    #[envconfig(from = "NTFY_URL", default = "https://ntfy.sh")]
    pub ntfy_url: String,

    #[envconfig(from = "NTFY_TOPIC")]
    pub ntfy_topic: Option<String>,

    #[envconfig(from = "NTFY_USERNAME")]
    pub ntfy_username: Option<String>,

    #[envconfig(from = "NTFY_PASSWORD")]
    pub ntfy_password: Option<String>,
}

pub fn get() -> Config {
    debug!("Initializing configuration");
    Config::init_from_env().unwrap()
}