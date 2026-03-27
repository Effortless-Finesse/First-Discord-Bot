use twilight_model::application::interaction::{Interaction, InteractionData};
use crate::context::Context;
use crate::dispatcher::{HandlerResult, InteractionHandlerRegistration};
use crate::utilities::functions::{has_guild_permission, parameter_to_int, parameter_to_string};
use std::sync::Arc;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use std::str::FromStr;

/*
impl CreateOption for u32 {
    fn create_option() -> ApplicationCommandOption {
        ApplicationCommandOption {
            name: "role_color".to_string(),
            description: "The color of the role".to_string(),
            kind: ApplicationCommandOptionType::Integer,
            required: false,
            autocomplete: false,
            channel_types: None,
            min_value: None,
            max_value: None,
            min_length: None,
            max_length: None,
            options: None,
            resolved: None,
        }
    }
}
*/

pub async fn create_role_command(ctx: Arc<Context>, interaction: Interaction) {
    if !has_guild_permission(ctx.clone(), &interaction).await {
        return;
    }

    if let Some(InteractionData::ApplicationCommand(command)) = &interaction.data {
        let role_name: String = parameter_to_string(command.options[0].value.clone()).unwrap();
        let role_color: Option<i64> = parameter_to_int(command.options[1].value.clone());
        let guild_id = interaction.guild_id.unwrap();
        
        match role_color {
            Some(color) => {
                let role = ctx.http.create_role(guild_id).name(&role_name).color(color as u32).await.expect("Could not create role"); 
            },
            None => {
                let role = ctx.http.create_role(guild_id).name(&role_name).await.expect("Could not create role"); 
            }
        }

        let data = InteractionResponseDataBuilder::new()
            .content("Role created successfully.")
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
    
}

pub fn create_role_command_handler(ctx: Arc<Context>, interaction: Interaction) -> HandlerResult {
    Box::pin(create_role_command(ctx, interaction))
}

inventory::submit! {
    InteractionHandlerRegistration {
        name: "create_role",
        handler: create_role_command_handler,
    }
}

/*
#[inventory::make(InteractionHandlerRegistration)]
pub static CREATE_ROLE_COMMAND: InteractionHandlerRegistration = InteractionHandlerRegistration {
    name: "create_role",
    handler: create_role_command_handler,
};
*/