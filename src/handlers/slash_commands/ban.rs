use twilight_model::application::interaction::Interaction;
use crate::context::Context;
use crate::dispatcher::{HandlerResult, InteractionHandlerRegistration};
use crate::utilities::functions::{has_guild_permission, get_pinged_user_id};
use std::sync::Arc;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

pub async fn ban_command(ctx: Arc<Context>, interaction: Interaction) {
    if !has_guild_permission(ctx.clone(), &interaction).await {
        return;
    }

    let target_user_id = match get_pinged_user_id(&interaction) {
        Some(id) => id,
        None => return,
    };

    let guild_id = match interaction.guild_id {
        Some(id) => id,
        None => return,
    };

    match ctx.http.create_ban(guild_id, target_user_id).await {
        Ok(_) => {
            let data = InteractionResponseDataBuilder::new()
                .content("User has been banned successfully.")
                .build();
            let response = InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(data),
            };
            let _ = ctx.http
                .interaction(ctx.application_id)
                .create_response(interaction.id, &interaction.token, &response)
                .await;
        },
        Err(err) => {
            eprintln!("Failed to ban user: {:?}", err);
        }
    }
}

pub fn ban_command_handler(ctx: Arc<Context>, interaction: Interaction) -> HandlerResult {
    Box::pin(ban_command(ctx, interaction))
}

inventory::submit! {
    InteractionHandlerRegistration {
        name: "ban",
        handler: ban_command_handler
    }
}
