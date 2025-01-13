use twilight_model::id::Id;

use crate::models::animated_emoji::AniEmoji;

pub mod color {
    pub const PRIMARY: u32 = 0xccff77;
    pub const ERROR: u32 = 0xff3355;
}

pub mod limit {
    pub const MAX_DICE: i64 = 30;
    pub const MAX_CLOW: i64 = 5;
}

pub const DICE: &[AniEmoji] = &[
    AniEmoji(Id::new(1322123824287711242)), // 1
    AniEmoji(Id::new(1322123836774289418)), // 1
    AniEmoji(Id::new(1322123899114094652)), // 1
    AniEmoji(Id::new(1322123914792538163)), // 2
    AniEmoji(Id::new(1322123925227831296)), // 2
    AniEmoji(Id::new(1322123940767731812)), // 2
    AniEmoji(Id::new(1322123948812537970)), // 3
    AniEmoji(Id::new(1322123957859782738)), // 3
    AniEmoji(Id::new(1322123970840887399)), // 3
    AniEmoji(Id::new(1322123979741331487)), // 4
    AniEmoji(Id::new(1322123988956348426)), // 4
    AniEmoji(Id::new(1322124002109689927)), // 4
    AniEmoji(Id::new(1322124013547425792)), // 5
    AniEmoji(Id::new(1322124022925758564)), // 5
    AniEmoji(Id::new(1322124031612289055)), // 5
    AniEmoji(Id::new(1322124040051232798)), // 6
    AniEmoji(Id::new(1322124049245012049)), // 6
    AniEmoji(Id::new(1322124059869184021)), // 6
];
