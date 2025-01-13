use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

#[derive(Default)]
pub struct SeedGenerator(FxHasher);

pub enum TimeHash {
    Minute,
    // Hour,
    Second,
    Day,
}

impl SeedGenerator {
    pub fn hash<H: Hash>(mut self, item: H) -> Self {
        item.hash(&mut self.0);
        self
    }
    pub fn specific_time(kind: TimeHash) -> u64 {
        const SECOND: u64 = 1;
        const MINUTE: u64 = 60 * SECOND;
        const HOUR: u64 = 60 * MINUTE;
        const DAY: u64 = 24 * HOUR;
        let unix_secs = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        let mut mod_unix_secs = unix_secs;
        let sub = match kind {
            TimeHash::Minute => MINUTE,
            TimeHash::Day => {
                // Make GMT+7 the root
                mod_unix_secs += 7 * HOUR;
                DAY
            }
            TimeHash::Second => SECOND,
        };
        unix_secs - mod_unix_secs % sub
    }
    pub fn hash_time(self, kind: TimeHash) -> Self {
        let unix_whole = Self::specific_time(kind);
        self.hash(unix_whole)
    }
    pub fn finish(self) -> u64 {
        self.0.finish()
    }
}
