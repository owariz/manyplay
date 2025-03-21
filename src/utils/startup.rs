use poise::serenity_prelude as serenity;
use lavalink_rs::prelude::LavalinkClient;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn startup(
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<LavalinkClient, Error>,
) -> Result<(), Error> {
    let guild_id_str = std::env::var("GUILD_ID").map_err(|_| "Missing GUILD_ID in environment")?;
    let guild_id = serenity::GuildId::new(
        guild_id_str
            .parse()
            .map_err(|_| "GUILD_ID must be a valid integer")?,
    );

    let existing_commands = guild_id
        .get_commands(&ctx.http)
        .await
        .map_err(|e| format!("Failed to get existing commands: {}", e))?;
    for cmd in existing_commands {
        guild_id
            .delete_command(&ctx.http, cmd.id)
            .await
            .map_err(|e| format!("Failed to delete command {}: {}", cmd.name, e))?;
    }

    poise::builtins::register_in_guild(&ctx, &framework.options().commands, guild_id)
        .await
        .map_err(|e| format!("Failed to register commands in guild: {}", e))?;

    println!(
        "{}#{} is online!",
        ready.user.name,
        ready.user.discriminator.unwrap()
    );

    Ok(())
}