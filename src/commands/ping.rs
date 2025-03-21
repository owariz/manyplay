use poise::serenity_prelude as serenity;
use poise::CreateReply;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let now = serenity::Timestamp::now().unix_timestamp();
    let created_at = ctx.created_at().unix_timestamp();

    let api_latency = (now - created_at) * 1000;

    let embed = serenity::CreateEmbed::new()
        .title("Pong!")
        .field("", format!("> **API Latency** ```{} ms```", api_latency), false)
        .color(0x00ff00);

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}