use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn change_reason(
    ctx: Context<'_>,
    #[description = "The case ID to update"] case_id: u32,
    #[description = "The new reason"] new_reason: String,
) -> Result<(), Error> {
    ctx.say(format!("change_reason command! {}", case_id))
        .await?;

    Ok(())
}
