use twilight_model::guild::Permissions;
use twilight_model::application::interaction::Interaction;
use twilight_model::id::Id;
use twilight_model::id::marker::{RoleMarker, UserMarker};
use twilight_model::application::interaction::InteractionData;
use twilight_model::application::interaction::application_command::CommandOptionValue;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::channel::message::MessageFlags;
use twilight_util::builder::InteractionResponseDataBuilder;
use std::sync::Arc;
use crate::context::Context;
use crate::macros;

#[macro_use]
use crate::send_priv_msg;

pub async fn has_guild_permission(ctx: Arc<Context>, interaction: &Interaction) -> bool {
    if let Some(permissions) = &interaction.member.as_ref().and_then(|m| m.permissions) {
        let is_mod = permissions.contains(Permissions::MANAGE_MESSAGES);
        let is_admin = permissions.contains(Permissions::ADMINISTRATOR);

        if is_mod || is_admin {
            return true;
        }

        send_priv_msg!(ctx, interaction, "You don't have permission to perform this action.");

        return false;

    } else {
        return false;
    }
}

/*
pub fn get_pinged_user_id(interaction: &Interaction) -> Option<Id<UserMarker>> {
    if let Some(data) = &interaction.data {
        if let twilight_model::application::interaction::InteractionData::ApplicationCommand(command) = data {
            for option in &command.options {
                if let twilight_model::application::interaction::application_command::CommandOptionValue::User(user_id) = option.value {
                    return Some(user_id);
                }
            }
        }
    }
    None
}
*/

pub fn get_pinged_user_id(interaction: &Interaction) -> Option<Id<UserMarker>> {
    // 1. Safely access the InteractionData
    let data = interaction.data.as_ref()?;

    // 2. Safely downcast to ApplicationCommand
    let command = match data {
        InteractionData::ApplicationCommand(cmd) => cmd,
        _ => return None, // Not a slash command
    };

    // 3. Check the parameter containing target ResolvedUser
    for option in &command.options {
        if let CommandOptionValue::User(user_id) = option.value {
            return Some(user_id);
        }
    }

    None
}

/*
fn get_command_parameter(command: ApplicationCommand, parameter: usize) -> CommandOptionValue {
        match command.options[parameter].value {
            CommandOptionValue::Attachment(a) => a,
            CommandOptionValue::Boolean(b) => b,
            CommandOptionValue::Channel(c) => c,
            CommandOptionValue::Focused(d) => d,
            CommandOptionValue::Integer(e) => e,
            CommandOptionValue::Mentionable(f) => f,
            CommandOptionValue::Number(g) => g,
            CommandOptionValue::Role(h) => h,
            CommandOptionValue::String(i) => i,
            CommandOptionValue::SubCommand(j) => j,
            CommandOptionValue::SubCommandGroup(k) => k,
            CommandOptionValue::User(l) => l,
        }
    }
    */

pub fn parameter_to_string(parameter: CommandOptionValue) -> Option<String> {
    match parameter {
        CommandOptionValue::String(s) => Some(s),
        _ => None,
    }
}

pub fn parameter_to_int(parameter: CommandOptionValue) -> Option<i64> {
    match parameter {
        CommandOptionValue::Integer(i) => Some(i),
        _ => None,
    }
}

pub fn parameter_to_user_id(parameter: CommandOptionValue) -> Option<Id<UserMarker>> {
    match parameter {
        CommandOptionValue::User(user_id) => Some(user_id),
        _ => None,
    }
}

pub fn parameter_to_role_id(parameter: CommandOptionValue) -> Option<Id<RoleMarker>> {
    match parameter {
        CommandOptionValue::Role(role) => Some(role),
        _ => None,
    }
}