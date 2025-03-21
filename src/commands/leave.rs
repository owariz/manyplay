// /// Leave the current voice channel.
// #[poise::command(slash_command, prefix_command)]
// pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
//     let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
//     let lava_client = ctx.data().clone();
//     let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

//     lava_client.delete_player(guild_id).await?;

//     if manager.get(guild_id).is_some() {
//         manager.remove(guild_id).await?;
//     }

//     ctx.say("Left voice channel.").await?;
//     Ok(())
// }