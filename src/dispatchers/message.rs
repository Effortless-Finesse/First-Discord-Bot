
fn handle_message_create(message: Message) {
    println!("Received message: {}", message.content);

    match message.content.as_str() {
        
        "!ping" => {
            // Respond with "Pong!"
            println!("Pong!");
        },

        "!hello" => {
            // Respond with a greeting
            println!("Hello, {}!", message.author.name);
        },

        s if s.starts_with("!echo ") => {
            // Echo the message back
            let echo_content = &s[6..];
            println!("{}", echo_content);
        },
        

        //add commands related to moderation, fun, utility, etc.
        // Intention to create two seperate submodules: normal commands and privileged commands

        _ => {
            // Handle other messages or ignore
        }
    }
}