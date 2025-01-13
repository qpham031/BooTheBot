mod data;
mod interaction;
mod message;
mod response_data;

use data::Data;
use response_data::ResponseData;
use std::future::IntoFuture;
use twilight_http::response::{marker::EmptyBody, ResponseFuture};
use twilight_model::{
    application::interaction::Interaction, channel::Message, http::interaction::InteractionResponse,
};

use crate::models::app_state::AppState;

#[derive(Debug)]
pub struct Handler<'a> {
    data: Data<'a>,
    raw: InputRaw<'a>,
    state: AppState,
}

#[derive(Debug, Clone, Copy)]
pub enum InputRaw<'a> {
    Message(&'a Message),
    Interaction(&'a Interaction),
}

impl<'a> Handler<'a> {
    pub fn new(state: AppState, raw: impl Into<InputRaw<'a>>) -> Self {
        let raw = raw.into();
        let data = Data::from(raw);
        Self { data, raw, state }
    }
    pub fn response_message_future(self) -> Option<ResponseFuture<Message>> {
        let InputRaw::Message(msg) = self.raw else {
            return None;
        };
        if matches!(self.data, Data::None) {
            return None;
        }
        let channel_id = msg.channel_id;
        let message_id = msg.id;
        let ResponseData {
            content,
            embeds,
            components,
            ..
        } = self.data.into();

        Some(
            self.state
                .bot
                .create_message(channel_id)
                .reply(message_id)
                .content(&content)
                .embeds(&embeds)
                .components(&components)
                .into_future(),
        )
    }
    #[allow(unused)]
    pub fn response_interaction(self) -> InteractionResponse {
        ResponseData::from(self.data).into()
    }
    pub fn response_interaction_future(self) -> Option<ResponseFuture<EmptyBody>> {
        let InputRaw::Interaction(itr) = self.raw else {
            return None;
        };
        let response = ResponseData::from(self.data).into();
        let Interaction {
            id: interaction_id,
            application_id,
            token: interaction_token,
            ..
        } = itr;

        Some(
            self.state
                .bot
                .interaction(*application_id)
                .create_response(*interaction_id, interaction_token, &response)
                .into_future(),
        )
    }
}
