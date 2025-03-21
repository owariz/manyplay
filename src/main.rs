use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use songbird::SongbirdKey; // เปลี่ยน SerenityInit เป็น SongbirdKey

mod commands;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: utils::load_commands::load_commands().await,
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                ctx.data.write().await.insert::<SongbirdKey>(songbird::Songbird::serenity()); // ใช้ SongbirdKey
                utils::startup::startup(ctx, ready, framework).await?;
                Ok(())
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}