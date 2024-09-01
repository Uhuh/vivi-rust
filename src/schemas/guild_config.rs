use bson::{doc, oid::ObjectId};
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId, RoleId};

use crate::MongoConfig;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GuildConfig {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub guild_id: GuildId,
    pub max_warns: usize,
    /// Warnings expire when their creation date exceeds the life span.
    pub warn_life_span: usize,
    pub server_log_whitelist: Vec<ChannelId>,
    pub ban_message: Option<String>,
    pub banned_words: Vec<String>,
    pub join_roles: Option<Vec<RoleId>>,
    #[serde(rename = "modLog")]
    pub mod_log_channel_id: Option<ChannelId>,
    #[serde(rename = "serverLog")]
    pub server_log_channel_id: Option<ChannelId>,
    #[serde(rename = "muteRole")]
    pub mute_role_id: Option<RoleId>,
    #[serde(rename = "modRole")]
    pub mod_role_id: Option<RoleId>,
}

pub fn create_new_guild_config(guild_id: GuildId) -> GuildConfig {
    GuildConfig {
        id: None,
        guild_id,
        ban_message: Some(String::new()),
        banned_words: vec![String::new()],
        join_roles: Some(vec![]),
        max_warns: 3,
        mod_log_channel_id: None,
        server_log_channel_id: None,
        mod_role_id: None,
        mute_role_id: None,
        server_log_whitelist: vec![],
        warn_life_span: 7,
    }
}

pub async fn get_guild_config(
    guild_id: GuildId,
    mongo_config: &MongoConfig,
) -> anyhow::Result<GuildConfig> {
    let guild_configs = mongo_config
        .database
        .collection::<GuildConfig>("guildconfigs");

    let guild_config = guild_configs
        .find_one(doc! { "guildId": guild_id.to_string() })
        .await?;

    if let Some(guild_config) = guild_config {
        Ok(guild_config)
    } else {
        let guild_config = create_new_guild_config(guild_id);

        let _ = guild_configs.insert_one(guild_config.clone()).await?;

        Ok(guild_config)
    }
}
