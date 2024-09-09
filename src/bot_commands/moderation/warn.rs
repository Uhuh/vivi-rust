use crate::bot_commands::command_helpers::mod_check;
use crate::schemas::{get_guild_config, get_users_active_warns, Case, CaseType};
use crate::{Context, Error};
use chrono::{TimeDelta, Utc};
use poise::serenity_prelude::{self as serenity, CreateMessage};
use poise::CreateReply;

#[poise::command(slash_command, prefix_command, aliases("warn"), check = "mod_check")]
pub async fn warn_user(
    ctx: Context<'_>,
    #[description = "User to warn"] user: serenity::User,
    #[description = "The reason for the warning"] reason: String,
) -> Result<(), Error> {
    let Some(guild) = ctx.guild().map(|guild| guild.clone()) else {
        return Err("Failed to grab Guild for warn_user.".into());
    };

    let data = ctx.data();
    let mongo_config = &data.mongo_config;

    let guild_config = get_guild_config(guild.id, &mongo_config.database).await?;
    #[allow(clippy::cast_possible_wrap)]
    // warn_life_span will be guarded by the configuration command.
    let expiration_date = Utc::now() - TimeDelta::days(guild_config.warn_life_span as i64);

    let active_warns =
        get_users_active_warns(guild.id, user.id, expiration_date, &mongo_config.database).await?;

    let _ = ctx
        .say(format!(
            "{} currently has {} active warnings.",
            user.name,
            active_warns.len()
        ))
        .await?;

    let case = Case::new(
        guild.id,
        user.id,
        ctx.author().id,
        reason.clone(),
        CaseType::Warn,
    );

    case.save(&mongo_config.database).await?;
    case.announce_to_mod_logs(&ctx, &mongo_config.database)
        .await?;

    let message = format!("You've been warned in **{}**\n\n**Reason**: {reason}", guild.name);
    let builder = CreateMessage::new().content(message);
    let _ = user.dm(ctx.http(), builder).await;

    if active_warns.len() + 1 >= guild_config.max_warns {
        let builder = CreateMessage::default().content(String::from(
            "Due to exceeding the warn limit, you've been banned.",
        ));
        let _ = user.dm(ctx.http(), builder).await;
        let message = CreateReply::default().ephemeral(true);

        let _ = match guild.ban(ctx.http(), &user, 0).await {
            Ok(()) => ctx.send(message.content(format!("Banned user {}", user.name))).await,
            Err(_) => ctx.send(message.content(format!("Failed to ban user {}.", user.name))).await,
        };
    }

    Ok(())
}
