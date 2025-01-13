use super::data::{
    About, BookOfAnswers, ClowCardInfo, Data, Dice, DrawClowcard, Error, RandomPick,
    RelationshipCalculator,
};
use crate::{
    constants::{self, color},
    models::{
        clow_cards::ClowCardDeck,
        relationship_level::RelationshipLevel,
        seed_generator::{SeedGenerator, TimeHash},
    },
};
use rand::seq::SliceRandom;
use std::fmt::Write;
use std::{borrow::Cow, sync::LazyLock};
use tracing::warn;
use twilight_mention::{
    timestamp::{Timestamp, TimestampStyle},
    Mention,
};
use twilight_model::{
    channel::message::{Component, Embed, MessageFlags},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::embed::EmbedBuilder;

#[derive(Default)]
pub struct ResponseData<'a> {
    pub content: Cow<'a, str>,
    pub embeds: Vec<Embed>,
    pub components: Vec<Component>,
    pub ephemeral: bool,
}

impl From<Data<'_>> for ResponseData<'_> {
    fn from(value: Data) -> Self {
        match value {
            Data::RandomPick(inner) => inner.into(),
            Data::BookOfAnswers(inner) => inner.into(),
            Data::DrawClowcard(inner) => inner.into(),
            Data::ClowCardInfo(inner) => inner.into(),
            Data::Dice(inner) => inner.into(),
            Data::LoveCalculator(inner) => inner.into(),
            Data::About(inner) => inner.into(),
            Data::None => Default::default(),
            Data::Error(inner) => inner.into(),
        }
    }
}

impl From<RandomPick<'_>> for ResponseData<'_> {
    fn from(value: RandomPick) -> Self {
        // Choose
        let mut rng = rand::thread_rng();
        let Some(the_one) = value.choices.choose(&mut rng) else {
            return Default::default();
        };

        // Determine prefix & postfix of each choice when display
        let line_list = value.choices.iter().any(|c| c.len() > 40);
        let (prefix, postfix) = if line_list {
            // List
            ("\n1. ", "")
        } else {
            // Inline
            (" ", ";")
        };

        let mut content = String::new();
        // Prompt
        if value.show_prompt {
            content.push_str("**Prompt:**");
            value.choices.iter().for_each(|choice| {
                content.push_str(prefix);
                content.push_str(choice);
                content.push_str(postfix);
            });
            if !line_list {
                content.pop();
            }
            content.push('\n');
            content.push('\n');
        }
        // Result
        content.push_str("**Em chọn:**");
        if the_one.contains('\n') {
            content.push('\n');
        } else {
            content.push(' ');
        }
        content.push_str(the_one);
        let content = content.into();

        Self {
            content,
            ..Default::default()
        }
    }
}

impl From<BookOfAnswers<'_>> for ResponseData<'_> {
    fn from(value: BookOfAnswers) -> Self {
        use crate::models::book_of_answers::BookOfAnswers;

        let author = value.author.expect("author should always be present");
        let quote = BookOfAnswers::draw(value.prompt, author);

        let content = if value.prompt.is_some() && value.show_prompt {
            let prompt = value.prompt.unwrap();
            format!("**Prompt:** {prompt}\n>>> {quote}").into()
        } else {
            quote.into()
        };

        Self {
            content,
            ..Default::default()
        }
    }
}

impl From<DrawClowcard<'_>> for ResponseData<'_> {
    fn from(value: DrawClowcard<'_>) -> Self {
        let DrawClowcard {
            prompt,
            author,
            amount,
            show_prompt,
        } = value;
        let author = author.expect("An author must be included in DrawClowcard");

        let content = if prompt.is_none() && amount.is_none() {
            // Daily
            let unix = SeedGenerator::specific_time(TimeHash::Day);
            let next = Timestamp::new(unix, Some(TimestampStyle::ShortDate));
            format!(
                "Thẻ bài Clow của {} hôm nay ({})",
                author.mention(),
                next.mention()
            )
        } else if prompt.is_some() && show_prompt {
            // Random w/prompt
            format!("**Prompt:** {}", prompt.unwrap())
        } else {
            // Random w/o prompt
            let amount = amount.unwrap_or(1);
            format!("{} vừa rút ngẫu nhiên {} thẻ bài", author.mention(), amount)
        }
        .into();

        let (embeds, components) = ClowCardDeck::draw(prompt, author, amount);

        Self {
            content,
            embeds,
            components,
            ..Default::default()
        }
    }
}
impl From<ClowCardInfo<'_>> for ResponseData<'_> {
    fn from(value: ClowCardInfo<'_>) -> Self {
        let content = ClowCardDeck::long_by_name(&value.name)
            .map(Cow::from)
            .unwrap_or_default();

        if content.is_empty() {
            warn!("unknown ClowCard: `{}`", value.name);
        }

        Self {
            content,
            ephemeral: true,
            ..Default::default()
        }
    }
}
impl From<Dice> for ResponseData<'_> {
    fn from(value: Dice) -> Self {
        let mut rng = rand::thread_rng();
        let mut content = String::new();
        for _ in 0..value.amount {
            let die = constants::DICE
                .choose(&mut rng)
                .expect("DICE is always valid");
            let _ = write!(&mut content, "{die} ");
        }
        let content = content.into();

        Self {
            content,
            ..Default::default()
        }
    }
}
impl From<RelationshipCalculator> for ResponseData<'_> {
    fn from(value: RelationshipCalculator) -> Self {
        let [user1, user2] = value
            .targets
            .into_inner()
            .expect("RelaCalc always find a way to extract 2 user ids");
        let embeds = RelationshipLevel::embed(user1, user2);
        let content = format!(
            "Mối quan hệ giữa {} và {} hiện đang là..",
            user1.mention(),
            user2.mention()
        )
        .into();
        ResponseData {
            embeds,
            content,
            ..Default::default()
        }
    }
}
impl From<About> for ResponseData<'_> {
    fn from(_value: About) -> Self {
        static ABOUT: LazyLock<Box<str>> = LazyLock::new(|| {
            std::fs::read_to_string("static/about.md")
                .map(Into::into)
                .unwrap_or_default()
        });

        let embeds = vec![EmbedBuilder::new()
            .title("About")
            .description(&**ABOUT)
            .color(constants::color::PRIMARY)
            .build()];
        Self {
            embeds,
            ..Default::default()
        }
    }
}
impl From<Error> for ResponseData<'_> {
    fn from(value: Error) -> Self {
        let embeds = vec![EmbedBuilder::new()
            .description(value.error)
            .color(color::ERROR)
            .build()];
        Self {
            embeds,
            ephemeral: true,
            ..Default::default()
        }
    }
}

impl From<ResponseData<'_>> for InteractionResponse {
    fn from(value: ResponseData) -> Self {
        InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                content: Some(value.content.into_owned()),
                embeds: Some(value.embeds),
                components: Some(value.components),
                flags: value.ephemeral.then_some(MessageFlags::EPHEMERAL),
                allowed_mentions: Some(Default::default()),
                ..Default::default()
            }),
        }
    }
}
