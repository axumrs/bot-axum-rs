use std::sync::Arc;

use axum::{routing::post, Router};
use axum_tg_bot::Bot;
use bot_axum_rs::{handler, model::AppState, Config};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = Config::from_env().expect("配置失败");
    let github_webhook_url = cfg.github.webhook_url.clone();
    let axumrs_webhook_url = cfg.axumrs.webhook_url.clone();

    let bot = axum_tg_bot::Bot {
        chat_id: cfg.bot.chat_id,
        token: cfg.bot.token.clone(),
    };
    let bot = Arc::new(bot);

    let state = Arc::new(AppState {
        bot: bot.clone(),
        github_cfg: cfg.github,
        axumrs_cfg: cfg.axumrs,
    });

    if cfg.auto_set_bot_webhook {
        tokio::spawn(register_webhook(bot, cfg.bot.full_webhook_url().clone()));
    }

    let app = Router::new()
        .route(&cfg.bot.webhook_url, post(handler::bot_handler))
        .route(&axumrs_webhook_url, post(handler::axum_rs_handler))
        .route(&github_webhook_url, post(handler::github_hook_handler))
        .with_state(state);

    tracing::info!(
        "服务器监听于：{}\n - TG bot: {} \n - Axum.rs: {} \n - Github: {}",
        &cfg.web.addr,
        &cfg.bot.webhook_url,
        &axumrs_webhook_url,
        &github_webhook_url
    );

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn register_webhook(bot: Arc<Bot>, url: String) {
    let res = bot.del_webhook().await.unwrap();
    tracing::info!("删除webhook: {}", res);

    let res = bot.set_webhook(&url).await.unwrap();
    tracing::info!("设置webhook: {}, {}", res, &url);
}
