use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "SSH_LOGS_PATH", default = "/var/log/auth.log")]
    pub ssh_logs_path: String,
    
    #[envconfig(from = "AUTH_SUCCESS_REGEX", default = r"Accepted (\w+) for (\w+) from ([\d\.]+) port ")]
    pub auth_sucess_regex: String,
    
    #[envconfig(from = "PUSHOVER_API_KEY")]
    pub pushover_api_key: String,

    #[envconfig(from = "PUSHOVER_USER_KEY")]
    pub pushover_user_key: String,
}

pub fn init() -> Config {
    Config::init_from_env().unwrap()
}