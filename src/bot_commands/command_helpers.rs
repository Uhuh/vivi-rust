use crate::schemas::get_guild_config;
use crate::{Context, Error};

/// Check if the user has the `mod_role_id` for the guild they're using a command in.
pub async fn mod_check(ctx: Context<'_>) -> Result<bool, Error> {
    ctx.defer_ephemeral().await?;

    let Some(guild_id) = ctx.guild_id() else {
        println!("Message did not originate from guild");
        return Ok(false);
    };
    
    let data = ctx.data();

    let config = get_guild_config(guild_id, &data.mongo_config).await?;

    let Some(mod_role_id) = config.mod_role_id else {
        println!("Guild does not have mod role configured.");
        return Ok(false);
    };

    let has_role = ctx
        .author()
        .has_role(&ctx, guild_id, &mod_role_id)
        .await?;

    Ok(has_role)
}
