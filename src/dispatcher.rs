use std::collections::HashMap;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use inventory;
use twilight_model::application::interaction::{Interaction, InteractionData};
use crate::context::Context;

pub type HandlerResult = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type InteractionHandler = fn(Arc<Context>, Interaction) -> HandlerResult;

inventory::collect!(InteractionHandlerRegistration);

pub struct InteractionHandlerRegistration {
    pub name: &'static str,
    pub handler: InteractionHandler,
}

/*
pub struct InteractionHandlerRegistration {
    pub command: CommandModel,
    pub handler: InteractionHandler,
}
*/

#[derive(Clone)]
pub struct InteractionDispatcher
{
    commands: HashMap<String, InteractionHandler>
}

impl InteractionDispatcher
{
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register_command(&mut self, name: String, handler: InteractionHandler) {
        self.commands.insert(name, handler);
    }

    pub async fn dispatch(&self, ctx: Arc<Context>, interaction: Interaction) {
        
        match &interaction.data {
            Some(InteractionData::ApplicationCommand(command)) => {
                if let Some(handler) = self.commands.get(&command.name) {
                    handler(ctx.clone(), interaction).await;
                } else {
                    // Handle unknown command
                }
            }
            _ => {
                // Handle other interaction types if needed
            }
        }
    }
}

