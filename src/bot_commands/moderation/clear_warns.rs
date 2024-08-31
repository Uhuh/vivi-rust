use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn clear_warns(
    ctx: Context<'_>,
    #[description = "The user to remove a warning from."] user: serenity::User,
    #[description = "The reason for removing the warning."] reason: Option<String>,
) -> Result<(), Error> {
    ctx.say(format!("clear_warns command! {}", user.name))
        .await?;

    Ok(())
}
