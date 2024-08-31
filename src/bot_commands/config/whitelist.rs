use serenity::all::standard::CommandResult;
use serenity::all::Message;
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

#[command]
pub async fn whitelist(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Whitelist command! : )").await?;

    Ok(())
}