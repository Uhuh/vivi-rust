use crate::bot_commands::command_helpers::mod_check;
use crate::schemas::{Case, CaseType};
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity};
use poise::CreateReply;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn kick_user(
    ctx: Context<'_>,
    #[description = "The user to kick"] user: serenity::User,
    #[description = "The reason for the kick"] reason: String,
) -> Result<(), Error> {
    let Some(guild) = ctx.guild().map(|guild| guild.clone()) else {
        return Err("Failed to grab Guild for warn_user.".into());
    };

    let data = ctx.data();
    let mongo_config = &data.mongo_config;

    let case = Case::new(guild.id, user.id, ctx.author().id, reason.clone(), CaseType::Kick);

    let message = CreateReply::default().ephemeral(true);

    let _ = match guild.kick_with_reason(ctx, &user, &reason).await {
        Ok(()) => {
            case.save(&mongo_config.database).await?;
            case.announce_to_mod_logs(&ctx, &mongo_config.database).await?;
            ctx.send(message.content(format!("Kicked {} from the server.", user.name))).await?
        },
        Err(err) => {
            println!("Failed to kick user: {} from server: {}. Err: {err}", user.name, guild.id);
            ctx.send(message.content(format!("Failed to kick {} from the server.", user.name))).await?
        }
    };

    Ok(())
}
