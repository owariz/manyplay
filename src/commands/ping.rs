use poise::serenity_prelude as serenity;
use poise::CreateReply;
use std::time::Instant;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let message = ctx.send(
        CreateReply::default()
            .content("Measuring ping...")
            .ephemeral(false)
    ).await?;
    
    let now = serenity::Timestamp::now().unix_timestamp();
    let created_at = ctx.created_at().unix_timestamp();
    let api_latency = (now - created_at) * 1000;
    
    let start_time = Instant::now();
    let http_client = ctx.serenity_context().http.clone();
    
    let bot_latency = {
        let channel_id = ctx.channel_id();
        let _ = http_client.get_channel(channel_id).await;
        start_time.elapsed().as_millis()
    };
    
    let embed = serenity::CreateEmbed::new()
        .title("🏓 Pong!")
        .field("Bot Latency", format!("> ```{} ms```", bot_latency), false)
        .field("API Latency", format!("> ```{} ms```", api_latency), false)
        .color(0x00ff00)
        .timestamp(serenity::Timestamp::now());
    
    message.edit(
        ctx, 
        CreateReply::default()
            .embed(embed)
            .content("")
    ).await?;
    
    Ok(())
}