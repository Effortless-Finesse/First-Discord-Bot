use twilight_model::application::interaction::Interaction;
use crate::context::Context;
use crate::dispatcher::{HandlerResult, InteractionHandlerRegistration};
use crate::utilities::functions::{has_guild_permission, get_pinged_user_id};
use std::sync::Arc;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

pub async fn mute_command(ctx: Arc<Context>, interaction: Interaction) {
    if !has_guild_permission(ctx.clone(), &interaction).await {
        return;
    }

    let target_user_id = match get_pinged_user_id(&interaction) {
        Some(id) => id,
        None => return,
    };

    match ctx.http.create_private_channel(target_user_id).await {
        Ok(channel_resp) => {
            if let Ok(dm_channel) = channel_resp.model().await {
                let data = InteractionResponseDataBuilder::new()
                    .content("You have been muted successfully.")
                    .build();
                let response = InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(data),
                };
                let _ = ctx.http
                    .interaction(ctx.application_id)
                    .create_response(interaction.id, &interaction.token, &response)
                    .await;
                
                // Note: The original code didn't actually send a DM, 
                // it just responded to the interaction. 
                // If you want to actually send a message to the DM channel:
                // let _ = ctx.http.create_message(dm_channel.id).content("Hello!").await;
            }
        }
        Err(err) => {
            eprintln!("Failed to create private channel: {:?}", err);
        }
    }
}

pub fn mute_command_handler(ctx: Arc<Context>, interaction: Interaction) -> HandlerResult {
    Box::pin(mute_command(ctx, interaction))
}

inventory::submit! {
    InteractionHandlerRegistration {
        name: "mute",
        handler: mute_command_handler
    }
}