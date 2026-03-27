use twilight_model::application::interaction::{Interaction, InteractionData};
use crate::context::Context;
use crate::dispatcher::{HandlerResult, InteractionHandlerRegistration};
use crate::utilities::functions::{has_guild_permission, parameter_to_string};
use std::sync::Arc;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use twilight_model::channel::message::MessageFlags;

use crate::send_priv_msg;

pub async fn whisper_command(ctx: Arc<Context>, interaction: Interaction) {
    if !has_guild_permission(ctx.clone(), &interaction).await {
        return;
    }

    let target_user: String = match interaction.data {
        Some(InteractionData::ApplicationCommand(command)) => {
            parameter_to_string(command.options[0].value.clone()).unwrap()
        },
        _ => return,
    };

    send_priv_msg!(ctx, interaction, target_user);

    let data = InteractionResponseDataBuilder::new()
        .content("Message sent successfully.")
        .build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data),
    };

    let _ = ctx.http
        .interaction(ctx.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await;
}

pub fn whisper_command_handler(ctx: Arc<Context>, interaction: Interaction) -> HandlerResult {
    Box::pin(whisper_command(ctx, interaction))
}

inventory::submit! {
    InteractionHandlerRegistration {
        name: "whisper",
        handler: whisper_command_handler
    }
}