use crate::bot_commands::command_helpers::mod_check;
use crate::schemas::{create_new_case, get_guild_config, get_users_active_warns};
use crate::{Context, Error};
use chrono::{TimeDelta, Utc};
use poise::serenity_prelude::{self as serenity, CreateMessage};

#[poise::command(slash_command, prefix_command, aliases("warn"), check = "mod_check")]
pub async fn warn_user(
    ctx: Context<'_>,
    #[description = "User to warn"] user: serenity::User,
    #[description = "The reason for the warning"] reason: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let guild = ctx.guild().unwrap().clone();

    if user.bot {
        ctx.say("You're warning a bot.").await?;
    }

    let data = ctx.data();
    let mongo_config = &data.mongo_config;

    let guild_config = get_guild_config(guild_id, mongo_config).await?;
    let expiration_date = Utc::now() - TimeDelta::days(guild_config.warn_life_span.into());

    let active_warns =
        get_users_active_warns(guild_id, user.id, expiration_date, &mongo_config.database).await?;

    ctx.say(format!(
        "{} currently has {} active warnings.",
        user.name,
        active_warns.len()
    ))
    .await?;

    create_new_case(
        guild_id,
        user.id,
        ctx.author().id,
        &reason,
        crate::schemas::CaseType::Warn,
        &mongo_config.database,
    )
    .await?;

    if active_warns.len() + 1 >= guild_config.max_warns.try_into().unwrap() {
        let message = format!(
            "You've been warned in **{}**\nDue to exceeding the warn limit, you've been banned.\n\n**Reason**: {}",
            guild.name,
            reason
        );
        let builder = CreateMessage::new().content(message);
        let _ = user.dm(ctx.http(), builder).await;

        let ban = guild.ban(ctx.http(), &user, 0).await;

        match ban {
            Err(e) => println!("Failed to ban user {}; {:?}", user.name, e),
            Ok(()) => println!("Banned user {}", user.name),
        }
    }

    Ok(())
}
