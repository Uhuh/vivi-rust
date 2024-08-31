use bson::{doc, oid::ObjectId, Document, Uuid};
use chrono::{DateTime, Local, Utc};
use futures::TryStreamExt;
use mongodb::Database;
use poise::serenity_prelude::{GuildId, MessageId, UserId};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Case {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    case_type: CaseType,
    case_id: Uuid,
    guild_id: GuildId,
    user_id: UserId,
    mod_id: UserId,
    reason: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    punishment_length: chrono::DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    creation_date: chrono::DateTime<Utc>,
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
            "case_type": CaseType::Warn.to_string()
        })
        .await?;

    Ok(warns.try_collect().await?)
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

pub async fn create_new_case(
    guild_id: GuildId,
    user_id: UserId,
    mod_id: UserId,
    reason: &String,
    case_type: CaseType,
    database: &Database,
) -> anyhow::Result<()> {
    let case = Case {
        id: None,
        case_id: Uuid::new(),
        case_type,
        guild_id,
        user_id,
        mod_id,
        reason: reason.clone(),
        creation_date: Utc::now(),
        // @TODO - Figure out how to make this optional
        punishment_length: Utc::now(),
    };

    let _ = database.collection("cases")
        .insert_one(case)
        .await?;

    Ok(())
}
