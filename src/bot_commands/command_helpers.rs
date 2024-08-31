use crate::schemas::get_guild_config;
use crate::{Context, Error};

/// Check if the user has the mod_role_id for the guild they're using a command in.
pub async fn mod_check(
    ctx: Context<'_>,
) -> Result<bool, Error> {
    ctx.defer_ephemeral().await?;

    if ctx.guild_id().is_none() {
        println!("Message did not originate from guild");
        return Ok(false);
    }

    let guild_id = ctx.guild_id().unwrap();
    let data = ctx.data();

    let config = get_guild_config(guild_id, &data.mongo_config).await.unwrap();

    if let Some(mod_role_id) = config.mod_role_id {
        let has_role = ctx
            .author()
            .has_role(&ctx, guild_id, &mod_role_id)
            .await
            .unwrap();

        if has_role {
            Ok(true)
        } else {
            println!("User lacks permissions. {:?}", ctx.author().global_name);
            Ok(false)
        }
    } else {
        println!("Guild does not have mod role configured.");
        Ok(false)
    }
}
