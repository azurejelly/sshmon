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
    
    #[envconfig(from = "DISCORD_WEBHOOK")]
    pub discord_webhook: Option<String>,
}

pub fn get() -> Config {
    debug!("Initializing configuration");
    Config::init_from_env().unwrap()
}