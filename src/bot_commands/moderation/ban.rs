use crate::bot_commands::command_helpers::mod_check;
use crate::schemas::{Case, CaseType};
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateMessage};
use poise::CreateReply;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn ban_user(
    ctx: Context<'_>,
    #[description = "The user to ban."] user: serenity::User,
    #[description = "The reason for the ban."] reason: String,
    #[description = "Delete X days worth of messages."] days: Option<u8>,
) -> Result<(), Error> {
    let Some(guild) = ctx.guild().map(|guild| guild.clone()) else {
        return Err("Failed to grab Guild for warn_user.".into());
    };

    let data = ctx.data();
    let mongo_config = &data.mongo_config;

    let case = Case::new(
        guild.id,
        user.id,
        ctx.author().id,
        reason.clone(),
        CaseType::Ban,
    );

    let message = format!("You've been banned in **{}**\n\n**Reason**: {reason}", guild.name);
    let builder = CreateMessage::new().content(message);
    let _ = user.dm(ctx, builder).await;

    let message = CreateReply::default().ephemeral(true);
    let _ = match guild.ban(ctx, &user, days.unwrap_or_default()).await {
        Ok(()) => {
            case.save(&mongo_config.database).await?;
            case.announce_to_mod_logs(&ctx, &mongo_config.database)
                .await?;
            ctx.send(message.content(format!("Banned user {}", user.name))).await
        },
        Err(_) => ctx.send(message.content(format!("Failed to ban user {}. Aborting creating a case.", user.name))).await,
    };
    
    Ok(())
}
