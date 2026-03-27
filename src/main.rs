use twilight_gateway::{Intents, Shard, ShardId, Event, EventTypeFlags, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::{MessageCreate, Ready};
use futures::{StreamExt, TryStreamExt};

use dotenvy::dotenv;

use serde_json::Value;

use std::env;
use crate::init::init;
use crate::context::Context;
use std::sync::Arc;

mod init;
mod context;
mod dispatcher;
mod utilities;
pub mod handlers;
pub mod commands;
pub mod macros;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let client = HttpClient::new(token.clone());
    let mut shard = Shard::new(ShardId::ONE, token.clone(), Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT);
    
    let application_id = client.current_user_application().await?.model().await?.id;

    let ctx = Arc::new(Context {
        http: client,
        application_id,
    });

    let dispatcher = init(ctx.clone()).await.unwrap();
    
    /*
    let mut events = shard.into_stream();
    while let Some(item) = events.next().await {
        let message: String = match item {
            Ok(message) => message.to_string(),
            Err(err) => {
                let err: twilight_gateway::error::ReceiveMessageError = err;
                eprintln!("Error receiving event: {:?}", err);
                continue;
            }
        };

     let event: Event = match serde_json::from_str(&message) {
            Ok(event) => event,
            Err(err) => {
                eprintln!("Error parsing message: {:?}", err);
                continue;
            }
        };
    */

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        
        
        let event = match item {
            Ok(event) => event,
            Err(err) => {
                let err: twilight_gateway::error::ReceiveMessageError = err;
                eprintln!("Error receiving event: {:?}", err);
                continue;
            }
        };
       

        match event {
            Event::GatewayHeartbeatAck => continue,
            Event::MessageCreate(message) => {
                println!("Received message: {}", message.content);
            },
            Event::InteractionCreate(interaction) => {
                let dispatcher = dispatcher.clone();
                let ctx = ctx.clone();
                tokio::spawn(async move {
                    dispatcher.dispatch(ctx, interaction.0).await;
                });
            },
            _ => {}
        }
    }

    Ok(())
}
