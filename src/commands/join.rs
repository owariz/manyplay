use poise::CreateReply;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let member_id = ctx.author().id;
    
    let channel_id = ctx
        .guild()
        .ok_or("Must be in a guild")?
        .voice_states
        .get(&member_id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or("You are not in a voice channel")?;
    
    let manager = songbird::get(ctx.serenity_context()).await
        .ok_or("Songbird Voice client not initialized")?;
    
    let handler_result = manager.join(guild_id, channel_id).await;
    
    match handler_result {
        Ok(handler) => {
            let mut handler = handler.lock().await;
            if let Err(e) = handler.deafen(true).await {
                ctx.send(CreateReply::default()
                    .content(format!("Joined voice channel but failed to deafen bot: {}", e)))
                    .await?;
            } else {
                ctx.send(CreateReply::default()
                    .content("Joined voice channel and deafened bot!"))
                    .await?;
            }
        }
        Err(e) => {
            ctx.send(CreateReply::default()
                .content(format!("Failed to join voice channel: {}", e)))
                .await?;
        }
    }
    
    Ok(())
}
