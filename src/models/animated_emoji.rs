use std::fmt::Display;

use twilight_model::id::{marker::EmojiMarker, Id};

#[derive(Clone, Copy)]
pub struct AniEmoji(pub Id<EmojiMarker>);

impl Display for AniEmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<a:a:{}>", self.0.get())
    }
}
