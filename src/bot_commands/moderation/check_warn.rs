use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn check_warns(
    ctx: Context<'_>,
    #[description = "The user to check the warnings of."] user: serenity::User,
    #[description = "Only check none expired warnings"] is_active: Option<bool>,
) -> Result<(), Error> {
    ctx.say(format!("check_warns command! {}", user.name))
        .await?;

    Ok(())
}
