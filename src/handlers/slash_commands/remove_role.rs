use twilight_model::application::interaction::{Interaction, InteractionData};
use crate::context::Context;
use crate::dispatcher::{HandlerResult, InteractionHandlerRegistration};
use crate::utilities::functions::{get_pinged_user_id, has_guild_permission, parameter_to_role_id};
use std::sync::Arc;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;
use std::eprintln;

pub async fn remove_role_command(ctx: Arc<Context>, interaction: Interaction) {
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
    
    if let Some(InteractionData::ApplicationCommand(command)) = &interaction.data {

        let role_id = parameter_to_role_id(command.options[1].value.clone()).expect("Could not fetch Role ID");
    


        match ctx.http.remove_guild_member_role(guild_id, target_user_id, role_id).await {
            Ok(_) => {
                let data = InteractionResponseDataBuilder::new()
                    .content("Role removed successfully.")
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
                eprintln!("Failed to remove role: {:?}", err);
            }
        }
    }
}

pub fn remove_role_command_handler(ctx: Arc<Context>, interaction: Interaction) -> HandlerResult {
    Box::pin(remove_role_command(ctx, interaction))
}


inventory::submit! {
    InteractionHandlerRegistration {
        name: "removerole",
        handler: remove_role_command_handler,
    }
}