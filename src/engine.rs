use crate::models::app_state::AppState;
use crate::{commands::CommandRegister, handler::Handler};
use tracing::{info, warn};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt};
use twilight_model::gateway::payload::incoming::{InteractionCreate, MessageCreate};

pub struct Engine {
    state: AppState,
    shard: Shard,
}

impl Engine {
    pub async fn new(token: impl Into<String>) -> anyhow::Result<Self> {
        let token = token.into();
        let state = AppState::new_with_token(token.clone()).await?;
        let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT;
        let shard = Shard::new(ShardId::ONE, token, intents);

        Ok(Self { state, shard })
    }
    pub async fn run(self) {
        let Self { state, mut shard } = self;

        CommandRegister::new(state.clone()).register().await;

        let wanted_event_types = EventTypeFlags::READY
            | EventTypeFlags::MESSAGE_CREATE
            | EventTypeFlags::INTERACTION_CREATE;

        while let Some(item) = shard.next_event(wanted_event_types).await {
            let Ok(event) = item else {
                warn!(source = ?item.unwrap_err(), "fail to recieve event");
                continue;
            };

            match event {
                Event::MessageCreate(msg) => {
                    // The author should not be a bot
                    // The content should be accessible
                    if msg.author.bot || msg.content.is_empty() {
                        continue;
                    }

                    tokio::spawn(Self::message_create(state.clone(), msg));
                }
                Event::InteractionCreate(itr) => {
                    tokio::spawn(Self::interaction_create(state.clone(), itr));
                }
                Event::Ready(ready) => {
                    info!("{} is ready!", ready.user.name);
                }
                _ => {}
            }
        }
    }

    async fn message_create(state: AppState, mut msg: Box<MessageCreate>) {
        let mention_str = state.info.mention.as_ref();
        if msg.content.starts_with(mention_str) {
            // SAFETY: `mention_str` is a ASCII string,
            // and we replace it with ASCII characters
            let bytes = unsafe { msg.0.content.as_bytes_mut() };
            bytes[0] = b'~';
            bytes[1..mention_str.len()].fill(b' ');
        }

        // Parse command into future response
        if let Some(fut) = Handler::new(state, &*msg).response_message_future() {
            let _ = fut
                .await
                .inspect_err(|err| warn!(?err, "unable to reponse message command"));
        }
    }

    async fn interaction_create(state: AppState, itr: Box<InteractionCreate>) {
        // Parse command into future response
        if let Some(fut) = Handler::new(state, &*itr).response_interaction_future() {
            let _ = fut
                .await
                .inspect_err(|err| warn!(?err, "unable to reponse interaction command"));
        } else {
            warn!("unable to parse command")
        }
    }
}
