mod commands;
mod constants;
mod engine;
mod extensions;
mod handler;
mod models;

use engine::Engine;

#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt().init();

    let token = std::env::var("DISCORD_TOKEN").expect("env var `DISCORD_TOKEN` should exists");

    let engine = Engine::new(token)
        .await
        .expect("Engine should be sucessfully initialized");
    engine.run().await;
}
