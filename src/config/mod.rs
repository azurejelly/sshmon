use envconfig::Envconfig;

use crate::notifier::NotifierType;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "SSH_LOGS_PATH", default = "/var/log/auth.log")]
    pub ssh_logs_path: String,
    
    #[envconfig(from = "AUTH_SUCCESS_REGEX", default = r"Accepted (\w+) for (\w+) from ([\d\.]+) port ")]
    pub auth_sucess_regex: String,

    #[envconfig(from = "DEBUG", default = "false")]
    pub debug: bool,

    #[envconfig(from = "NOTIFIER", default = "pushover")]
    pub notifier: NotifierType,

    #[envconfig(from = "PUSHOVER_API_KEY")]
    pub pushover_api_key: Option<String>,

    #[envconfig(from = "PUSHOVER_USER_KEY")]
    pub pushover_user_key: Option<String>,

    #[envconfig(from = "HOSTNAME_OVERRIDE")]
    pub hostname_override: Option<String>,
}

pub fn get() -> Config {
    Config::init_from_env().unwrap()
}