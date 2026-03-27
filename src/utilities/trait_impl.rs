/*
use twilight_model::application::interaction::application_command::CommandOptionValue;

impl From<CommandOptionValue> for i64 {
    fn from(value: CommandOptionValue) -> Self {
        match value {
            CommandOptionValue::Integer(i) => i,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<CommandOptionValue> for String {
    fn from(value: CommandOptionValue) -> Self {
        match value {
            CommandOptionValue::String(s) => s,
            _ => panic!("Invalid value"),
        }
    }
}
*/