use anyhow::anyhow;
use std::str::FromStr;
use tracing::warn;
use twilight_model::{
    application::{command::CommandType, interaction::InteractionContextType},
    oauth::ApplicationIntegrationType,
};
use twilight_util::builder::command::{CommandBuilder, IntegerBuilder, StringBuilder, UserBuilder};

use crate::{constants::limit, models::app_state::AppState};

const RANDOM_PICK: CommandNamePair = CommandNamePair {
    names: &["pick", "choose"],
    id: Marker::RandomPick,
};
const CLOW_CARDS: CommandNamePair = CommandNamePair {
    names: &["drawclow", "dc"],
    id: Marker::DrawClowcard,
};
const RELA_CALC: CommandNamePair = CommandNamePair {
    names: &["relacalc", "lc"],
    id: Marker::RelationshipCalculator,
};
const DICE: CommandNamePair = CommandNamePair {
    names: &["dice"],
    id: Marker::Dice,
};
const BOOK_OF_ANSWERS: CommandNamePair = CommandNamePair {
    names: &["bookofanswers", "boa"],
    id: Marker::BookOfAnswers,
};
const ABOUT: CommandNamePair = CommandNamePair {
    names: &["about"],
    id: Marker::About,
};
const CMD_NAMES: &[CommandNamePair] = &[
    RANDOM_PICK,
    CLOW_CARDS,
    RELA_CALC,
    DICE,
    BOOK_OF_ANSWERS,
    ABOUT,
];

struct CommandNamePair<'a> {
    names: &'a [&'a str],
    id: Marker,
}

#[derive(Debug, Clone, Copy)]
pub enum Marker {
    RandomPick,
    BookOfAnswers,
    DrawClowcard,
    RelationshipCalculator,
    Dice,
    About,
}

impl FromStr for Marker {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CMD_NAMES
            .iter()
            .find_map(|pair| pair.names.contains(&s).then_some(pair.id))
            .ok_or_else(|| anyhow!("unknown CommandName: `{s}`"))
    }
}

pub struct CommandRegister(AppState);

impl CommandRegister {
    pub fn new(state: AppState) -> Self {
        Self(state)
    }
    pub async fn register(self) {
        // Draw Clow Cards
        let drawclow = CommandBuilder::new(
            CLOW_CARDS.names[0],
            "xem vận mệnh với thẻ bài Clow",
            CommandType::ChatInput,
        )
        .option(StringBuilder::new("prompt", "nội dung"))
        .option(
            IntegerBuilder::new("amount", "số lượng")
                .min_value(1)
                .max_value(limit::MAX_CLOW),
        )
        .build();

        // Randomly Pick
        let mut pick = CommandBuilder::new(
            RANDOM_PICK.names[0],
            "hỗ trợ bạn quyết định giữa muôn vàn sự lựa chọn",
            CommandType::ChatInput,
        )
        .build();
        pick.options = (0..25)
            .map(|idx| StringBuilder::new(format!("opt-{}", idx + 1), "thêm một lựa chọn").build())
            .collect();
        pick.options[0].required = Some(true);
        pick.options[1].required = Some(true);

        //Book Of Answers command
        let boa = CommandBuilder::new(
            BOOK_OF_ANSWERS.names[0],
            "hãy để cuốn sách này trả lời trăn trở của bạn",
            CommandType::ChatInput,
        )
        .option(StringBuilder::new("prompt", "nội dung"))
        .build();

        // Rolling Dice command
        let dice = CommandBuilder::new(DICE.names[0], "tung xúc sắc", CommandType::ChatInput)
            .option(
                IntegerBuilder::new("amount", "số lượng")
                    .min_value(1)
                    .max_value(limit::MAX_DICE),
            )
            .build();

        // Relationship Calculator command
        let relacalc = CommandBuilder::new(
            RELA_CALC.names[0],
            "kiểm tra sự kết nối giữa 2 users",
            CommandType::ChatInput,
        )
        .option(UserBuilder::new("user", "user").required(true))
        .option(UserBuilder::new("another_user", "another_user"))
        .build();

        // About command
        let about =
            CommandBuilder::new(ABOUT.names[0], "thông tin về bot", CommandType::ChatInput).build();

        // Adjust command scope
        let mut commands = [about, boa, dice, drawclow, pick, relacalc];
        commands.iter_mut().for_each(|cmd| {
            cmd.contexts = Some(vec![
                InteractionContextType::BotDm,
                InteractionContextType::Guild,
                InteractionContextType::PrivateChannel,
            ]);
            cmd.integration_types = Some(vec![
                ApplicationIntegrationType::GuildInstall,
                ApplicationIntegrationType::UserInstall,
            ]);
        });

        // Register commands
        let _ = self
            .0
            .bot
            .interaction(self.0.info.appid)
            .set_global_commands(&commands)
            .await
            .inspect_err(|err| warn!(?err, "fail to create commands"));
    }
}
