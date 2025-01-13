use std::ops::Deref;

use anyhow::Result;
use serde::Deserialize;
use twilight_mention::Mention;
use twilight_model::id::{
    marker::{ApplicationMarker, GenericMarker, UserMarker},
    Id,
};

type BotClient = twilight_http::Client;

#[derive(Debug)]
pub struct Bot(BotClient);

#[derive(Debug)]
pub struct BotInfo {
    pub appid: Id<ApplicationMarker>,
    pub mention: Box<str>,
}

impl Bot {
    pub fn new_with_token(token: impl Into<String>) -> Self {
        let client = BotClient::builder()
            .token(token.into())
            .default_allowed_mentions(Default::default())
            .build();
        Self(client)
    }
}

impl Deref for Bot {
    type Target = BotClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BotInfo {
    pub async fn init(bot: &Bot) -> Result<Self> {
        #[derive(Deserialize)]
        struct IdOnly {
            id: Id<GenericMarker>,
        }
        let bytes = bot.current_user().await?.bytes().await?;
        let appid = serde_json::from_slice::<IdOnly>(&bytes)?.id.cast();
        let mention = appid.cast::<UserMarker>().mention().to_string().into();
        Ok(Self { appid, mention })
    }
}
