
#[macro_export]
macro_rules! send_msg {
    ($ctx: ident, $interaction: ident, $msg: expr) => {
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseDataBuilder::new()
                .content($msg)
                .build()),
        };

        let _ = $ctx.http
            .interaction($ctx.application_id)
            .create_response($interaction.id, &$interaction.token, &response)
            .await;
    };
}

#[macro_export]
macro_rules! send_priv_msg {
    ($ctx: ident, $interaction: ident, $msg: expr) => {

        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseDataBuilder::new()
                .content($msg)
                .flags(MessageFlags::EPHEMERAL)
                .build()),
        };

        let _ = $ctx.http
            .interaction($ctx.application_id)
            .create_response($interaction.id, &$interaction.token, &response)
            .await;
    };
}

#[macro_export]
macro_rules! give_role {
    (/*$ctx: ident, $interaction: ident, */ $role: ident, $user: ident) => {
        
        user.roles.push($role);
    };
}

//I need to learn to handle data recieved as parameters from commands
#[macro_export]
macro_rules! send_dm {
    ($ctx: ident, $user_id: ident, $msg: expr) => {

            let channel_id = 
            match $ctx.http.create_private_channel($user_id).await {
                Ok(channel) => channel.model().await.unwrap().id,
                Err(err) => {
                    eprintln!("Error creating private channel: {:?}", err);
                    return;
                }
            };

        let _ = $ctx.http.create_message(channel_id).content($msg).await;
    };
}

/*
#[macro_export] //convert from a macro to an inline function
macro_rules! get_command_parameter {
    ($command: ident, $parameter: expr) => {
        match $command.options[$parameter].value {
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
    };
}
*/