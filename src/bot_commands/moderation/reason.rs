use bson::{doc, Uuid};
use poise::CreateReply;

use crate::bot_commands::command_helpers::mod_check;
use crate::schemas::update_case;
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command, check = "mod_check")]
pub async fn change_reason(
    ctx: Context<'_>,
    #[description = "The case ID to update"] case_id: String,
    #[description = "The new reason"] new_reason: String,
) -> Result<(), Error> {
    let case_id = Uuid::parse_str(case_id);

    let case_id = match case_id {
        Err(_) => return Err("Invalid case ID was provided.".into()),
        Ok(case_id) => case_id,
    };

    let data = ctx.data();
    let mongo_config = &data.mongo_config;

    let updated_case = update_case(
        case_id,
        doc! {
            "$set": { "reason": new_reason }
        },
        &mongo_config.database,
    ).await;

    let message = CreateReply::default().ephemeral(true);

    let updated_case = match updated_case {
        Ok(updated_case) => updated_case,
        Err(err) => {
            println!("Failed to update case. Err: {err}");
            let _ = ctx.send(message.content(format!("Failed to update case {case_id}"))).await?;
            return Err("Failed to update users case.".into());
        },
    };

    if let Some(updated_case) = updated_case {
        updated_case.announce_to_mod_logs(&ctx, &mongo_config.database).await?;
        let _ = ctx.send(message.content(format!("Successfully updated case {case_id}"))).await?;
    } else {
        let _ = ctx.send(message.content(format!("No cases exist with this id: {case_id}"))).await?;
    }

    Ok(())
}
