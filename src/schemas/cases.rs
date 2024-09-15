use bson::{doc, oid::ObjectId, Document, Uuid};
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::{options::{FindOneAndUpdateOptions, ReturnDocument}, Database};
use poise::serenity_prelude::{Color, CreateEmbed, CreateMessage, GuildId, Timestamp, UserId};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::{Context, Error};

use super::get_guild_config;

#[derive(Display, Serialize, Deserialize, Debug)]
pub enum CaseType {
    Mute,
    Warn,
    Ban,
    Kick,
    Unban,
    Unmute,
    Unwarn,
}

#[allow(clippy::struct_field_names)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Case {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub case_type: CaseType,
    pub case_id: Uuid,
    pub guild_id: GuildId,
    pub user_id: UserId,
    pub mod_id: UserId,
    pub reason: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub punishment_length: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub creation_date: DateTime<Utc>,
}

impl Case {
    pub fn new(
        guild_id: GuildId,
        user_id: UserId,
        mod_id: UserId,
        reason: String,
        case_type: CaseType,
    ) -> Self {
        Self {
            guild_id,
            user_id,
            mod_id,
            reason,
            case_type,
            id: None,
            case_id: Uuid::new(),
            creation_date: Utc::now(),
            punishment_length: Utc::now(),
        }
    }
}

impl Case {
    pub fn set_punishment_date(&mut self, punishment_date: DateTime<Utc>) {
        self.punishment_length = punishment_date;
    }

    pub async fn announce_to_mod_logs(
        &self,
        ctx: &Context<'_>,
        database: &Database,
    ) -> anyhow::Result<(), Error> {
        let Some(guild) = ctx.guild().map(|guild| guild.clone()) else {
            return Err("Missing guild".into());
        };

        let guild_config = get_guild_config(guild.id, database).await?;

        let channel = if let Some(mod_log_channel_id) = guild_config.mod_log_channel_id {
            guild.channels.get(&mod_log_channel_id)
        } else {
            None
        };

        match channel {
            Some(channel) => {
                let embed = create_case_embed(self);
                let message = CreateMessage::new().embed(embed);
                let _ = channel.send_message(&ctx, message).await?;
            }
            _ => println!(
                "Could not find guild mod channel {:?}",
                guild_config.mod_log_channel_id
            ),
        }

        Ok(())
    }

    pub async fn save(&self, database: &Database) -> anyhow::Result<()> {
        let _ = database
            .collection::<Case>("cases")
            .insert_one(self)
            .await?;

        Ok(())
    }
}

pub fn create_case_embed(case: &Case) -> CreateEmbed {
    CreateEmbed::new()
        .title(format!("{} | Case {}", case.case_type, case.case_id))
        .fields(vec![
            (
                "User",
                format!("{} (<@{}>)", case.user_id, case.user_id),
                true,
            ),
            ("Moderator", format!("<@{}>", case.user_id), true),
        ])
        .field("Reason", case.reason.clone(), false)
        .color(Color::new(6_573_123))
        .timestamp(Timestamp::now())
}

pub async fn update_case(
    case_id: Uuid,
    update_doc: Document,
    database: &Database
) -> anyhow::Result<Option<Case>> {
    Ok(database
        .collection::<Case>("cases")
        .find_one_and_update(doc! {
            "case_id": case_id
        }, update_doc)
        .return_document(ReturnDocument::After)
        .await?
    )
}

pub async fn get_user_cases(
    guild_id: GuildId,
    user_id: UserId,
    case_type: CaseType,
    database: &Database,
) -> anyhow::Result<Vec<Case>> {
    let cases = database
        .collection::<Case>("cases")
        .find(doc! {
            "user_id": user_id.to_string(),
            "guild_id": guild_id.to_string(),
            "case_type": case_type.to_string()
        })
        .await?;

    Ok(cases.try_collect().await?)
}

pub async fn get_users_active_warns(
    guild_id: GuildId,
    user_id: UserId,
    expiration_date: DateTime<Utc>,
    database: &Database,
) -> anyhow::Result<Vec<Case>> {
    let warns = database
        .collection::<Case>("cases")
        .find(doc! {
            "user_id": user_id.to_string(),
            "guild_id": guild_id.to_string(),
            "case_type": CaseType::Warn.to_string(),
            "creation_date": doc! {
                // Unsure as to why $gte works but $gt does NOT work.
                "$gte": expiration_date
            }
        })
        .await?;

    Ok(warns.try_collect().await?)
}

pub async fn get_user_warns(
    guild_id: GuildId,
    user_id: UserId,
    database: &Database,
) -> anyhow::Result<Vec<Case>> {
    let warns = database
        .collection::<Case>("cases")
        .find(doc! {
            "user_id": user_id.to_string(),
            "guild_id": guild_id.to_string(),
            "case_type": CaseType::Warn.to_string(),
        })
        .await?;

    Ok(warns.try_collect().await?)
}
