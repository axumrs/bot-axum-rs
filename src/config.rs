use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubConfig {
    pub secret_token: String,
    pub webhook_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AxumrsConfig {
    pub secret_token: String,
    pub webhook_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Webconfig {
    pub addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bot: axum_tg_bot::Config,
    pub github: GithubConfig,
    pub axumrs: AxumrsConfig,
    pub web: Webconfig,
    pub auto_set_bot_webhook: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
