use crate::{
    constants::color,
    models::{
        custom_id::CustomId,
        seed_generator::{SeedGenerator, TimeHash},
    },
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde::Deserialize;
use std::{borrow::Cow, ops::Deref, sync::LazyLock};
use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, Embed, EmojiReactionType,
    },
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

type Inner = Box<[ClowCard]>;

pub struct ClowCardDeck(Inner);

pub struct ClowCard {
    name: Box<str>,
    meaning: Box<str>,
    img_id: u64,
    full: Box<str>,
}

#[derive(Deserialize, Clone)]
struct ClowcardCow<'a> {
    id: u64,
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    meaning: Cow<'a, str>,
    #[serde(borrow)]
    message: Cow<'a, str>,
    #[serde(borrow)]
    warning: Cow<'a, str>,
}

impl ClowCardDeck {
    fn get_instance() -> &'static Self {
        static INSTANCE: LazyLock<ClowCardDeck> = LazyLock::new(|| {
            let raw = std::fs::read_to_string("static/ClowCardData.json").unwrap();
            let mut deck = serde_json::from_str::<Vec<ClowcardCow>>(&raw)
                .expect("ClowCardData should be in correct format")
                .into_iter()
                .map(
                    |ClowcardCow {
                         id,
                         name,
                         meaning,
                         message,
                         warning,
                     }| ClowCard {
                        full: format!(
                            "# [ The {name} ]\n\
                        ```md\n\
                        ## Ý NGHĨA\n{meaning}\n\n\
                        ## THÔNG ĐIỆP\n{message}\n\n\
                        ## CẢNH BÁO\n{warning}\n\
                        ```"
                        )
                        .into(),
                        name: name.into(),
                        meaning: meaning.into(),
                        img_id: id,
                    },
                )
                .collect::<Vec<_>>();
            deck.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));

            ClowCardDeck(deck.into())
        });
        &INSTANCE
    }
    pub fn draw(
        content: Option<&str>,
        author: Id<UserMarker>,
        amount: Option<usize>,
    ) -> (Vec<Embed>, Vec<Component>) {
        let deck = Self::get_instance();
        let time = match (content, amount) {
            (None, None) => TimeHash::Day,
            (None, Some(_)) => TimeHash::Second,
            (Some(_), _) => TimeHash::Minute,
        };
        let amount = amount.unwrap_or(1);
        let state = SeedGenerator::default()
            .hash_time(time)
            .hash(author)
            .hash(content)
            .hash(amount)
            .finish();
        let mut rng = StdRng::seed_from_u64(state);

        let mut embeds = Vec::with_capacity(amount);
        let mut components = Vec::with_capacity(amount);

        deck.choose_multiple(&mut rng, amount)
            .map(Self::short)
            .for_each(|(embed, component)| {
                embeds.push(embed);
                components.push(component);
            });
        let components = vec![Component::ActionRow(ActionRow { components })];

        (embeds, components)
    }
    fn short(
        ClowCard {
            name,
            meaning,
            img_id,
            ..
        }: &ClowCard,
    ) -> (Embed, Component) {
        const EMOJI_MAGICBOOK: EmojiReactionType = EmojiReactionType::Custom {
            animated: false,
            id: Id::new(1312304913455517737),
            name: None,
        };

        let custom_id = CustomId::ButtonClowcardInfo(Cow::Borrowed(name)).to_string();
        let title = format!("The {name}");
        let img = ImageSource::url(format!(
            "https://cdn.discordapp.com/attachments/953801841412538368/{img_id}/The{name}.jpg"
        ))
        .expect("valid img url");

        let embed = EmbedBuilder::new()
            .title(&title)
            .description(meaning.as_ref())
            .thumbnail(img)
            .color(color::PRIMARY)
            .build();
        let component = Component::Button(Button {
            custom_id: Some(custom_id),
            disabled: false,
            emoji: Some(EMOJI_MAGICBOOK),
            label: Some(title),
            style: ButtonStyle::Secondary,
            url: None,
            sku_id: None,
        });
        (embed, component)
    }
    fn long(ClowCard { full, .. }: &ClowCard) -> &str {
        full
    }
    pub fn long_by_name(name: &str) -> Option<&'static str> {
        let deck = Self::get_instance();
        let pos = deck
            .binary_search_by(|card| card.name.as_ref().cmp(name))
            .ok()?;
        Some(Self::long(&deck.0[pos]))
    }
}

impl Deref for ClowCardDeck {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
