use super::InputRaw;
use arrayvec::ArrayVec;
use std::borrow::Cow;
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Debug)]
pub enum Data<'a> {
    RandomPick(RandomPick<'a>),
    BookOfAnswers(BookOfAnswers<'a>),
    DrawClowcard(DrawClowcard<'a>),
    ClowCardInfo(ClowCardInfo<'a>),
    Dice(Dice),
    LoveCalculator(RelationshipCalculator),
    About(About),
    None,
    Error(Error),
}

#[derive(Debug)]
pub struct RandomPick<'a> {
    pub choices: Vec<&'a str>,
    pub show_prompt: bool,
}

#[derive(Debug)]
pub struct BookOfAnswers<'a> {
    pub prompt: Option<&'a str>,
    pub author: Option<Id<UserMarker>>,
    pub show_prompt: bool,
}

#[derive(Debug)]
pub struct Dice {
    pub amount: u32,
}

#[derive(Debug)]
pub struct DrawClowcard<'a> {
    pub prompt: Option<&'a str>,
    pub author: Option<Id<UserMarker>>,
    pub amount: Option<usize>,
    pub show_prompt: bool,
}
#[derive(Debug)]
pub struct ClowCardInfo<'a> {
    pub name: Cow<'a, str>,
}

#[derive(Debug)]
pub struct RelationshipCalculator {
    pub targets: ArrayVec<Id<UserMarker>, 2>,
}

#[derive(Debug)]
pub struct Error {
    pub error: String,
}

#[derive(Debug)]
pub struct About;

impl<'a> From<InputRaw<'a>> for Data<'a> {
    fn from(value: InputRaw<'a>) -> Self {
        match value {
            InputRaw::Message(message) => Data::from(message),
            InputRaw::Interaction(interaction) => Data::from(interaction),
        }
    }
}
