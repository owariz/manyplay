use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use songbird::SerenityInit;
use lavalink_rs::prelude::{LavalinkClient, NodeBuilder, NodeDistributionStrategy};
use lavalink_rs::model::{events::Events, UserId};

mod commands;
mod utils;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").map_err(|_| "Missing DISCORD_TOKEN in environment")?;

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_VOICE_STATES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: utils::load_commands::load_commands().await,
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                utils::startup::startup(ctx, ready, framework).await?;
                let events = Events::default();
                let node: NodeBuilder = NodeBuilder {
                    hostname: "lavahatry4.techbyte.host:3000".to_string(),
                    is_ssl: false,
                    password: "NAIGLAVA-dash.techbyte.host".to_string(),
                    events: events.clone(),
                    session_id: None,
                    user_id: UserId(ready.user.id.into()),
                };
                let nodes = vec![node];
                let lava_client = LavalinkClient::new(events, nodes, NodeDistributionStrategy::default()).await; // เพิ่ม .await เพื่อให้ได้ LavalinkClient โดยตรง

                Ok(lava_client)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await
        .map_err(|e| format!("Failed to create client: {}", e))?;

    client.start().await.map_err(|e| format!("Failed to start client: {}", e))?;

    Ok(())
}