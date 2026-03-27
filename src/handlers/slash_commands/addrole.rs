use twilight_model::application::interaction::{Interaction, InteractionData};
use crate::context::Context;
use crate::dispatcher::{HandlerResult, InteractionHandlerRegistration};
use crate::utilities::functions::{has_guild_permission, get_pinged_user_id};
use std::sync::Arc;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use twilight_model::application::interaction::ApplicationCommandOptionType;

pub async fn add_role_command(ctx: Arc<Context>, interaction: Interaction) {
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

    match ctx.http.add_guild_member_role(guild_id, target_user_id, role_id).await {
        Ok(_) => {
            let data = InteractionResponseDataBuilder::new()
                .content("Role added successfully.")
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
            eprintln!("Failed to add role: {:?}", err);
        }
    }
}

pub fn add_role_command_handler(ctx: Arc<Context>, interaction: Interaction) -> HandlerResult {
    Box::pin(add_role_command(ctx, interaction))
}

#[inventory::make(InteractionHandlerRegistration)]
pub static ADD_ROLE_COMMAND: InteractionHandlerRegistration = InteractionHandlerRegistration {
    name: "add_role",
    handler: add_role_command_handler,
};