use crate::bot_commands::command_helpers::mod_check;
use crate::{Context, Error};
use chrono::Local;
use duration_string::DurationString;
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn mute_user(
    ctx: Context<'_>,
    #[description = "The user to mute"] user: serenity::User,
    #[description = "The reason of the mute"] reason: String,
    #[description = "Duration of the mute"] duration: Option<String>,
) -> Result<(), Error> {
    ctx.say(format!("mute_user command! {}", user.name)).await?;

    if let Some(duration) = duration {
        let d = DurationString::from_string(duration).unwrap();
        println!("{:?}", d);
        ctx.say("duration or something").await?;
    }

    Ok(())
}
