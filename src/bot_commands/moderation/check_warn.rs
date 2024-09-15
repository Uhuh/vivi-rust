use crate::bot_commands::command_helpers::mod_check;
use crate::schemas::{get_guild_config, get_user_warns, get_users_active_warns, Case};
use crate::{Context, Error};
use chrono::{TimeDelta, Utc};
use poise::serenity_prelude::{self as serenity, CreateEmbed, UserId};
use poise::CreateReply;

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn check_warns(
    ctx: Context<'_>,
    #[description = "The user to check the warnings of."] user: serenity::User,
    #[description = "Only check none expired warnings"] is_active: Option<bool>,
) -> Result<(), Error> {
    let Some(guild) = ctx.guild().map(|guild| guild.clone()) else {
        return Err("Failed to grab Guild for warn_user.".into());
    };

    let data = ctx.data();
    let mongo_config = &data.mongo_config;

    let user_warns = if is_active.is_some() {
        let guild_config = get_guild_config(guild.id, &mongo_config.database).await?;
        #[allow(clippy::cast_possible_wrap)]
        // warn_life_span will be guarded by the configuration command.
        let expiration_date = Utc::now() - TimeDelta::days(guild_config.warn_life_span as i64);
        get_users_active_warns(guild.id, user.id, expiration_date, &mongo_config.database).await?
    } else {
        get_user_warns(guild.id, user.id, &mongo_config.database).await?
    };

    let embed = create_warns_embed(user.id, &user_warns);

    let message = CreateReply::default().embed(embed).ephemeral(true);

    let _ = ctx.send(message).await?;

    Ok(())
}

fn create_warns_embed(user_id: UserId, cases: &[Case]) -> CreateEmbed {
    let mut embed = CreateEmbed::new().title(format!("Cases for user_id: {user_id}"));

    if cases.is_empty() {
        return embed.description("This user has no warnings against them.");
    }

    embed = embed.description(format!("User (<@{user_id}>) has {} warn(s)\n\n", cases.len()));

    for case in cases {
        embed = embed.field(
            format!("Case {}", case.case_id),
            format!(
                "**Creation Date**: {}\n**Moderator:** <@{}>\n**Reason:** {}\n\n\n",
                case.creation_date, case.mod_id, case.reason
            ),
            false,
        );
    }

    embed
}
