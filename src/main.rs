#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::unwrap_used
)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::expect_used)]
#![forbid(unsafe_code)]

use bot_commands::{ban_user, change_reason, check_warns, clear_warns, kick_user, mute_user, unban, unmute, unwarn};
use mongodb::options::ClientOptions;
use mongodb::{Client as MongoClient, Database};
use poise::serenity_prelude::{self as serenity, async_trait, ChannelId, GuildId, Member, Message, MessageId, MessageUpdateEvent};
use poise::serenity_prelude::prelude::*;
use std::env;

use crate::bot_commands::warn_user;
mod bot_commands;
mod schemas;

struct MongoConfig {
    database: Database,
}

impl TypeMapKey for MongoConfig {
    type Value = Self;
}

pub struct Data {
    mongo_config: MongoConfig,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mongodb_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI");
    let mongodb_database = env::var("MONGODB_DATABASE").expect("You must set the MONGODB_DATABASE");

    let mongo_options = ClientOptions::parse(&mongodb_uri).await?;
    let mongo_config = MongoConfig {
        database: MongoClient::with_options(mongo_options)?.database(&mongodb_database),
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                warn_user(),
                unwarn(),
                unmute(),
                unban(),
                change_reason(),
                mute_user(),
                kick_user(),
                clear_warns(),
                check_warns(),
                ban_user(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { mongo_config })
            })
        })
        .build();

    let intents = GatewayIntents::all();
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await?;

    client.start().await?;

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, _ctx: poise::serenity_prelude::Context, new_member: Member) {

    }

    async fn message_update(&self, ctx: poise::serenity_prelude::Context, old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        println!("Old: {}  New: {}", old_if_available.is_some(), new.is_some());
    }

    async fn message_delete(&self, ctx: poise::serenity_prelude::Context, channel_id: ChannelId, deleted_message_id: MessageId, guild_id: Option<GuildId>) {
        println!("ChannelID: {channel_id}, MessageID: {deleted_message_id}, GuildID: {}", guild_id.unwrap());
    }
}
