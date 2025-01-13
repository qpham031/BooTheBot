use super::{
    data::{About, BookOfAnswers, Data, Dice, DrawClowcard, RandomPick, RelationshipCalculator},
    InputRaw,
};
use crate::commands::Marker;
use std::ops::Not;
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

pub struct ParseCommandError;

impl<'a> From<&'a MessageCreate> for InputRaw<'a> {
    fn from(value: &'a MessageCreate) -> Self {
        Self::from(&value.0)
    }
}

impl<'a> From<&'a Message> for InputRaw<'a> {
    fn from(value: &'a Message) -> Self {
        InputRaw::Message(value)
    }
}

impl<'a> From<&'a Message> for Data<'a> {
    fn from(value: &'a Message) -> Self {
        let Some(content) = value
            .content
            .as_str()
            .strip_prefix('~')
            .map(str::trim_start)
        else {
            return Data::None;
        };
        let (cmd, args) = content
            .split_once(' ')
            .map(|(cmd, args)| (cmd, args.trim_start()))
            .unwrap_or((content, ""));
        let author = value.author.id;

        let Ok(name) = cmd.parse() else {
            return Data::None;
        };

        (|| -> Result<Data<'_>, ParseCommandError> {
            Ok(match name {
                Marker::RandomPick => Data::RandomPick(args.try_into()?),
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
                    let mut boa: BookOfAnswers = args.try_into()?;
                    boa.author = Some(author);
                    Data::BookOfAnswers(boa)
                }
                Marker::Dice => Data::Dice(args.into()),
                Marker::About => Data::About(args.into()),
            })
        })()
        .unwrap_or(Data::None)
    }
}

impl<'a> TryFrom<&'a str> for RandomPick<'a> {
    type Error = ParseCommandError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let choices = value.split(';').map(str::trim).collect::<Vec<_>>();

        (choices.len() >= 2)
            .then_some(Self {
                choices,
                show_prompt: false,
            })
            .ok_or(ParseCommandError)
    }
}

impl<'a> TryFrom<&'a str> for BookOfAnswers<'a> {
    type Error = ParseCommandError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let input = value.is_empty().not().then_some(value);
        Ok(Self {
            prompt: input,
            author: None,
            show_prompt: false,
        })
    }
}

impl<'a> From<&'a str> for DrawClowcard<'a> {
    fn from(value: &'a str) -> Self {
        let mut splitting = value.splitn(2, ' ');
        let amount = splitting
            .next()
            .and_then(|amount| amount.parse().ok())
            .filter(|amount| *amount != 0);
        let prompt = amount
            .is_none()
            .then_some(value)
            .or(splitting.next())
            .filter(|value| !value.is_empty());
        Self {
            prompt,
            amount,
            author: None,
            show_prompt: false,
        }
    }
}

impl<'a> From<&'a str> for RelationshipCalculator {
    fn from(value: &'a str) -> Self {
        let targets = value
            .split_ascii_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_ascii_digit()))
            .filter_map(|s| s.parse().ok())
            .take(2)
            .collect();

        Self { targets }
    }
}

impl From<&str> for About {
    fn from(_value: &str) -> Self {
        Self
    }
}

impl From<&str> for Dice {
    fn from(value: &str) -> Self {
        let amount = value
            .parse()
            .ok()
            .filter(|amount| (1..=100).contains(amount))
            .unwrap_or(1);
        Self { amount }
    }
}
