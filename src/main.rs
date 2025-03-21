use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use songbird::SerenityInit;

mod commands;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: utils::load_commands::load_commands().await,
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                utils::startup::startup(ctx, ready, framework).await?;
                Ok(())
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await;
    client.unwrap().start().await.unwrap();
}