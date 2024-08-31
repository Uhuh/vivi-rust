use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn kick_user(
    ctx: Context<'_>,
    #[description = "The user to kick"] user: serenity::User,
    #[description = "The reason for the kick"] reason: Option<String>,
) -> Result<(), Error> {
    ctx.say(format!("kick_user command! {}", user.name)).await?;

    Ok(())
}
