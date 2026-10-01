use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::guild::Role;

trait identifiability
{
    fn get_name(&self) -> String;
}

#[derive(CreateCommand)]
#[command(name = "kick", desc = "Kick a user from the server")]
pub struct kick {
    #[command(desc = "The user to kick")]
    target_user: ResolvedUser, 
}

#[derive(CreateCommand)]
#[command(name = "ban", desc = "Ban a user from the server")]
pub struct ban {
    #[command(desc = "The user to ban")]
    target_user: ResolvedUser, 
}

#[derive(CreateCommand)]
#[command(name = "mute", desc = "Mute a server member")]
pub struct mute {
    #[command(desc = "The user to mute")]
    target_user: ResolvedUser, 
}

#[derive(CreateCommand)]
#[command(name = "unmute", desc = "Unmute a server member")]
pub struct unmute {
    #[command(desc = "The user to unmute")]
    target_user: ResolvedUser,
}

#[derive(CreateCommand)]
#[command(name = "whisper", desc = "Send a private message to a user")]
pub struct whisper {
    #[command(desc = "The user to whisper")]
    target_user: ResolvedUser, 
    #[command(desc = "The message to send")]
    message: String,
}

#[derive(CreateCommand)]
#[command(name = "addrole", desc = "Add a role to a user")]
pub struct addrole {
    #[command(desc = "The user to add the role to")]
    target_user: ResolvedUser, 
    #[command(desc = "The role to add")]
    role: Role,
}

#[derive(CreateCommand)]
#[command(name = "create_role", desc = "Create a new role")]
pub struct create_role {
    #[command(desc = "The name of the role")]
    role_name: String,
    #[command(desc = "Optional: Colour of the role")]
    role_color: Option<String>,
    //#[command(desc = "The color of the role")]
    //role_color: Option<i64>,
    
}

/*
impl identifiability for kick
{
    fn get_name(&self) -> String {
        "kick".to_string()
    }
}

impl identifiability for ban
{
    fn get_name(&self) -> String {
        "ban".to_string()
    }
}

impl identifiability for whisper
{
    fn get_name(&self) -> String {
        "whisper".to_string()
    }
}
*/

/*
impl CreateCommand for kick
{
    pub fn create_command() -> ApplicationCommandData {
        CommandModel::new("kick", "Kick a user from the server")
    }
}
*/