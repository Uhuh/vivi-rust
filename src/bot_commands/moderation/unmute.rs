use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "The user to unmute"] user: serenity::User,
) -> Result<(), Error> {
    ctx.say(format!("Unmute command! {}", user.name)).await?;

    Ok(())
}
