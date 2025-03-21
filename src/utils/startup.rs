use poise::serenity_prelude as serenity;
use dotenv::dotenv;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn startup(ctx: &serenity::Context, ready: &serenity::Ready, framework: &poise::Framework<(), Error>) -> Result<(), Error> {
    dotenv().ok();
    let guild_id_str = std::env::var("GUILD_ID").expect("missing GUILD_ID");
    let guild_id = serenity::GuildId::new(guild_id_str.parse().expect("GUILD_ID must be an integer"));

    let existing_commands = guild_id.get_commands(&ctx.http).await?;
    for cmd in existing_commands {
        guild_id.delete_command(&ctx.http, cmd.id).await?;
    }

    poise::builtins::register_in_guild(&ctx, &framework.options().commands, guild_id).await?;

    println!("{}#{} is online!", ready.user.name, ready.user.discriminator.unwrap());

    Ok(())
}