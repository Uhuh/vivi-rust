use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn unwarn(
    ctx: Context<'_>,
    #[description = "ID of the warn"] warn_id: String,
    #[description = "Reason for removing the warn"] reason: String,
) -> Result<(), Error> {
    ctx.say(format!("Unwarn command! {} {}", warn_id, reason))
        .await?;

    Ok(())
}
