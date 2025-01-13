use super::{
    data::{
        About, BookOfAnswers, Data, Dice, DrawClowcard, Error, RandomPick, RelationshipCalculator,
    },
    InputRaw,
};
use crate::{
    commands::Marker, handler::data::ClowCardInfo, models::custom_id::CustomId,
    extensions::CommandOptionValueData,
};
use twilight_model::{
    application::interaction::{
        application_command::{CommandData, CommandDataOption, CommandOptionValue},
        Interaction, InteractionData,
    },
    gateway::payload::incoming::InteractionCreate,
    id::{marker::UserMarker, Id},
};

impl<'a> From<&'a InteractionCreate> for InputRaw<'a> {
    fn from(value: &'a InteractionCreate) -> Self {
        Self::from(&value.0)
    }
}

impl<'a> From<&'a Interaction> for InputRaw<'a> {
    fn from(value: &'a Interaction) -> Self {
        InputRaw::Interaction(value)
    }
}

impl<'a> From<&'a Interaction> for Data<'a> {
    fn from(value: &'a Interaction) -> Self {
        fn app_cmd(data: &CommandData, author: Id<UserMarker>) -> Data {
            let args = data.options.as_slice();
            let Ok(name) = data.name.parse() else {
                return Data::None;
            };

            match name {
                Marker::RandomPick => Data::RandomPick(args.into()),
                Marker::DrawClowcard => {
                    let mut dc: DrawClowcard = args.into();
                    dc.amount = dc.amount.map(|a| a.min(5));
                    dc.author = Some(author);
                    Data::DrawClowcard(dc)
                }
                Marker::RelationshipCalculator => {
                    let mut lc: RelationshipCalculator = args.into();
                    let left = lc.targets.remaining_capacity();
                    (0..left).for_each(|_| lc.targets.push(author));
                    lc.targets.sort();
                    Data::LoveCalculator(lc)
                }
                Marker::BookOfAnswers => {
                    let mut boa: BookOfAnswers = args.into();
                    boa.author = Some(author);
                    Data::BookOfAnswers(boa)
                }
                Marker::Dice => Data::Dice(args.into()),
                Marker::About => Data::About(args.into()),
            }
        }

        fn msg_com(cid: CustomId) -> Data {
            match cid {
                CustomId::ButtonClowcardInfo(name) => Data::ClowCardInfo(ClowCardInfo { name }),
            }
        }

        let Some(data) = &value.data else {
            return Self::Error(Error {
                error: "unsupported command type".to_owned(),
            });
        };
        let author = value.author_id().expect("command should come from a user");

        match data {
            InteractionData::ApplicationCommand(data) => app_cmd(data, author),
            InteractionData::MessageComponent(data) => {
                let cid = CustomId::from(data.custom_id.as_str());
                msg_com(cid)
            }
            _ => Self::Error(Error {
                error: "unsupported command type".to_owned(),
            }),
        }
    }
}

impl<'a> From<&'a [CommandDataOption]> for RandomPick<'a> {
    fn from(value: &'a [CommandDataOption]) -> Self {
        let choices = value
            .iter()
            .map(|op| match &op.value {
                CommandOptionValue::String(choice) => choice.as_str(),
                _ => unreachable!("RandomPick takes strings only"),
            })
            .collect();
        Self {
            choices,
            show_prompt: true,
        }
    }
}

impl<'a> From<&'a [CommandDataOption]> for BookOfAnswers<'a> {
    fn from(value: &'a [CommandDataOption]) -> Self {
        let input = value.first().map(|op| match &op.value {
            CommandOptionValue::String(input) => input.as_str(),
            _ => unreachable!("BookOfAnswers takes a string only"),
        });
        Self {
            prompt: input,
            author: None,
            show_prompt: true,
        }
    }
}

impl<'a> From<&'a [CommandDataOption]> for DrawClowcard<'a> {
    fn from(value: &'a [CommandDataOption]) -> Self {
        let mut prompt = None;
        let mut amount = None;
        value.iter().for_each(|op| match op.name.as_str() {
            "prompt" => prompt = op.value.string(),
            "amount" => amount = op.value.usize(),
            _ => {}
        });
        Self {
            prompt,
            amount,
            author: None,
            show_prompt: true,
        }
    }
}

impl<'a> From<&'a [CommandDataOption]> for RelationshipCalculator {
    fn from(value: &'a [CommandDataOption]) -> Self {
        let targets = value
            .iter()
            .map(|op| match &op.value {
                CommandOptionValue::User(user) => *user,
                _ => unreachable!("LoveCalculator takes users only"),
            })
            .take(2)
            .collect();

        Self { targets }
    }
}

impl From<&[CommandDataOption]> for About {
    fn from(_value: &[CommandDataOption]) -> Self {
        Self
    }
}

impl From<&[CommandDataOption]> for Dice {
    fn from(value: &[CommandDataOption]) -> Self {
        let amount = value
            .first()
            .map(|op| match &op.value {
                CommandOptionValue::Integer(amount) => *amount,
                _ => unreachable!("Dice takes an integer"),
            })
            .unwrap_or(1);

        assert!((1..=100).contains(&amount));
        let amount = amount.try_into().expect("Dice takes a possitive integer");

        Dice { amount }
    }
}
