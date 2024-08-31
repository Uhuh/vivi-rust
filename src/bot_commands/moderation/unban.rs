use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user to unban"] user: serenity::User,
    #[description = "The reason for the unban"] reason: String,
) -> Result<(), Error> {
    ctx.say(format!("Unban command! {}", user.name)).await?;

    Ok(())
}
