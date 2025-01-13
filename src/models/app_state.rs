use super::*;
use anyhow::Result;
use bot::{Bot, BotInfo};
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState(Arc<AppStateInner>);

#[derive(Debug)]
pub struct AppStateInner {
    pub bot: Bot,
    pub info: BotInfo,
}

impl AppState {
    pub async fn new_with_token(token: impl Into<String>) -> Result<Self> {
        let bot = Bot::new_with_token(token.into());
        let info = BotInfo::init(&bot).await?;
        let inner = AppStateInner { bot, info };
        let state = Self(inner.into());
        Ok(state)
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
