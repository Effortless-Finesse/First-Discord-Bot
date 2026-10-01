use crate::dispatcher::{InteractionDispatcher, InteractionHandlerRegistration};
use crate::commands;
use std::sync::Arc;
use crate::context::Context;

use twilight_interactions::command::CreateCommand;

pub fn init(ctx: Arc<Context>) -> Result<InteractionDispatcher, ()> 
{
    let mut dispatcher = InteractionDispatcher::new();

    for registration in inventory::iter::<InteractionHandlerRegistration> {
        dispatcher.register_command(registration.name.to_string(), registration.handler.clone());
    }

    let commands = &[
        commands::kick::create_command().into(),
        commands::ban::create_command().into(),
        commands::whisper::create_command().into(),
        commands::create_role::create_command().into(),
        commands::addrole::create_command().into(),
        commands::mute::create_command().into(),
        commands::unmute::create_command().into(),
    ];

    ctx.http.interaction(ctx.application_id).set_global_commands(commands);

    Ok(dispatcher)
}
