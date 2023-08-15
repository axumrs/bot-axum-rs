use std::sync::Arc;

use axum_tg_bot::Bot;
use serde::{Deserialize, Serialize};

use crate::{AxumrsConfig, GithubConfig};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHook {
    pub repository: GithubHookRepository,
    pub head_commit: GithubHookCommit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHookRepository {
    pub full_name: String,
    pub html_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHookCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
}

impl GithubHookCommit {
    pub fn short_id(&self) -> String {
        let short_id = &self.id[..7];
        short_id.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubHookPusher {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AxumrsHook {
    pub title: String,
    pub url: String,
    pub subject_name: String,
}

pub struct AppState {
    pub bot: Arc<Bot>,
    pub github_cfg: GithubConfig,
    pub axumrs_cfg: AxumrsConfig,
}
