use twilight_model::application::interaction::application_command::CommandOptionValue;

pub trait CommandOptionValueData {
    fn string(&self) -> Option<&str>;
    fn i64(&self) -> Option<i64>;
    fn usize(&self) -> Option<usize>;
}

impl CommandOptionValueData for CommandOptionValue {
    fn string(&self) -> Option<&str> {
        match self {
            CommandOptionValue::String(value) => Some(value),
            _ => None,
        }
    }

    fn i64(&self) -> Option<i64> {
        match self {
            CommandOptionValue::Integer(value) => Some(*value),
            _ => None,
        }
    }

    fn usize(&self) -> Option<usize> {
        Self::i64(self).map(|value| value as usize)
    }
}
