use crate::models::seed_generator::{SeedGenerator, TimeHash};
use rand::{Rng, SeedableRng};
use twilight_model::{
    channel::message::Embed,
    id::{
        marker::{EmojiMarker, UserMarker},
        Id,
    },
};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

#[derive(Clone, Copy)]
pub struct RelationshipLevel {
    pub title: &'static str,
    pub description: &'static str,
    pub thumbnail: Id<EmojiMarker>,
    pub color: u32,
    pub upperbound: f32,
}
pub const RELATIONSHIP_LEVELS: &[RelationshipLevel] = &[
    RelationshipLevel {
        title: "[ Strangers in the Night ]",
        description: "Casual, brief encounter, no real connection.",
        thumbnail: Id::new(1323549313430851594),
        color: 0xD3D3D3,
        upperbound: 20.,
    },
    RelationshipLevel {
        title: "[ Social Snackers ]",
        description: "Friendly, light connection, often in social settings.",
        thumbnail: Id::new(1323551714887991296),
        color: 0xF1E2A7,
        upperbound: 50.,
    },
    RelationshipLevel {
        title: "[ Besties ]",
        description: "Solid friends, trust and fun, but not yet deeply emotional.",
        thumbnail: Id::new(1323551637075398700),
        color: 0x5D9BEC,
        upperbound: 75.,
    },
    RelationshipLevel {
        title: "[ Partners in Crime ]",
        description: "Strong bond, loyal and inseparable, lots of shared experiences.",
        thumbnail: Id::new(1323547947601891370),
        color: 0xFF6F61,
        upperbound: 90.,
    },
    RelationshipLevel {
        title: "[ Soulmates Forever ]",
        description: "Deep connection, unspoken understanding, and long-term commitment.",
        thumbnail: Id::new(1323550264824827924),
        color: 0x9B4D96,
        upperbound: 100.,
    },
];

impl RelationshipLevel {
    pub fn embed(user1: Id<UserMarker>, user2: Id<UserMarker>) -> Vec<Embed> {
        const NUM_BOXES: usize = 20;
        const VAL_BOX: usize = 100 / NUM_BOXES;
        const EMPTY: &str = "";

        let seed = SeedGenerator::default()
            .hash_time(TimeHash::Day)
            .hash(user1)
            .hash(user2)
            .finish();
        let mut rng = rand::prelude::StdRng::seed_from_u64(seed);
        let percent: f32 = rng.gen_range(0. ..100.);
        let num_fbox = percent.round() as usize / VAL_BOX;
        let num_ebox = NUM_BOXES - num_fbox;
        let RelationshipLevel {
            title,
            description,
            thumbnail,
            color,
            ..
        } = RELATIONSHIP_LEVELS
            .iter()
            .find(|lvl| percent <= lvl.upperbound)
            .cloned()
            .expect("percentage should be in range 0-100%");

        let description = format!(
            "{description}\n\
            ```css\n[{EMPTY:▣>num_fbox$}{EMPTY:▢>num_ebox$}] {percent:.2}%\n```"
        );
        let thumbnail = ImageSource::url(format!(
            "https://cdn.discordapp.com/emojis/{thumbnail}.webp"
        ))
        .unwrap();

        vec![EmbedBuilder::new()
            .title(title)
            .description(description)
            .thumbnail(thumbnail)
            .color(color)
            .build()]
    }
}
