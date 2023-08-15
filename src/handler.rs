use std::sync::Arc;

use axum::{extract::State, Json};
use axum_tg_bot::{TextMessage, Update};

use crate::model::{AppState, AxumrsHook, GithubHook};

pub async fn bot_handler(State(state): State<Arc<AppState>>, Json(update): Json<Update>) -> String {
    if let Some(msg) = update.message {
        let msg_text = msg.text.unwrap_or_default();
        let res = state
            .bot
            .send_text_msg(&TextMessage::new_text(&msg_text, state.bot.chat_id.into()))
            .await
            .unwrap();
        return res;
    }
    "".to_string()
}

pub async fn github_hook_handler(
    State(state): State<Arc<AppState>>,
    Json(github): Json<GithubHook>,
) -> String {
    // TODO: 验证签名

    let msg_text = format!(
        "🎉 天呀，劳模站长又双叒叕更新代码了！\n\n👉 {} @ {} \n\n📝 {}\n\n{}",
        github.head_commit.short_id(),
        github.repository.full_name,
        github.head_commit.message,
        github.head_commit.url,
    );

    state
        .bot
        .send_text_msg(&TextMessage::new_text(&msg_text, state.bot.chat_id.into()))
        .await
        .unwrap()
}

pub async fn axum_rs_handler(
    State(state): State<Arc<AppState>>,
    Json(data): Json<AxumrsHook>,
) -> String {
    // TODO: 验证签名

    let msg_text = format!(
        "👏 哇哦，勤劳的站长又双叒叕上新了！《{}》（专题：{}）\n{}",
        data.title, data.subject_name, data.url
    );
    state
        .bot
        .send_text_msg(&TextMessage::new_text(&msg_text, state.bot.chat_id.into()))
        .await
        .unwrap()
}
