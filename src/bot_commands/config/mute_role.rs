use serenity::all::standard::CommandResult;
use serenity::all::Message;
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

#[command]
pub async fn mute_role(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Mute role command! : )").await?;

    Ok(())
}