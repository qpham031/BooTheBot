use crate::models::seed_generator::{SeedGenerator, TimeHash};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{ops::Deref, sync::LazyLock};
use twilight_model::id::{marker::UserMarker, Id};

type Inner = Box<[Box<str>]>;

#[derive(Debug)]
pub struct BookOfAnswers(Inner);

impl BookOfAnswers {
    fn get_instance() -> &'static Self {
        static INSTANCE: LazyLock<BookOfAnswers> = LazyLock::new(|| {
            BookOfAnswers(
                std::fs::read_to_string("static/BookOfAnswers.txt")
                    .expect("`BookOfAnswers.txt` file should exist")
                    .split_inclusive("/*")
                    .map(str::trim_start)
                    .map(Into::into)
                    .collect::<Vec<Box<str>>>()
                    .into_boxed_slice(),
            )
        });
        &INSTANCE
    }

    pub fn draw(content: Option<&str>, author: Id<UserMarker>) -> &'static str {
        let book = Self::get_instance();
        match content {
            Some(content) => {
                let seed = SeedGenerator::default()
                    .hash_time(TimeHash::Minute)
                    .hash(author)
                    .hash(content)
                    .finish();
                let mut rng = StdRng::seed_from_u64(seed);
                book.0.choose(&mut rng)
            }
            None => {
                let mut rng = rand::thread_rng();
                book.0.choose(&mut rng)
            }
        }
        .expect("BookOfAnswers should not be empty")
    }
}

impl Deref for BookOfAnswers {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
