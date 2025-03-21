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

    if ctx
        .guild()
        .ok_or("Must be in a guild")?
        .voice_states
        .values()
        .any(|vs| vs.channel_id == Some(channel_id))
    {
        let manager = songbird::get(ctx.serenity_context()).await; // ลบ .expect()

        match manager {
            Some(manager) => {
                let manager = manager.clone();
                match manager.join(guild_id, channel_id).await {
                    Ok(_call) => {
                        ctx.send(CreateReply::default().content("Joined voice channel!")).await?;
                    }
                    Err(e) => {
                        ctx.send(CreateReply::default().content(format!("Failed to join voice channel: {}", e))).await?;
                    }
                }
            }
            None => {
                ctx.send(CreateReply::default().content("Songbird Voice client not initialized.")).await?;
            }
        }
    } else {
        ctx.send(CreateReply::default().content("No one is in the voice channel.")).await?;
    }

    Ok(())
}