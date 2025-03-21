use std::env;
use std::fs;
use toml::Value;
use chrono::Utc;
use chrono_tz::Asia::Bangkok;
use poise::serenity_prelude as serenity;
use poise::CreateReply;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    let developer_id: u64 = env::var("DEVELOPER_ID")?.parse()?;
    let developer = ctx.http().get_user(developer_id.into()).await?;
    let bot_name = ctx.serenity_context().cache.current_user().name.clone();
    let bot_avatar = ctx.serenity_context().cache.current_user().avatar_url().unwrap_or_default();
    let timestamp = Utc::now().with_timezone(&Bangkok).format("%Y-%m-%d %H:%M:%S").to_string();

    if ctx.serenity_context().http.broadcast_typing(ctx.channel_id().get().into()).await.is_ok() {
        let embed = serenity::CreateEmbed::new()
            .title(format!("Bot Information - {}", bot_name))
            .description("\nInformation about this bot\n")
            .fields(vec![
                ("Framework", "Poise", true),
                ("Language", format!("Rust: {}", get_rust_version()?).as_str(), true),
                ("Developer", developer.name.as_str(), true),
                (
                    "",
                    format!(
                        "> **Poise** {}\n > **Serenity** {}\n > **Tokio** {}\n > **Chrono** {}",
                        get_dependency_version("poise")?.as_str(),
                        get_dependency_version("serenity")?.as_str(),
                        get_dependency_version("tokio")?.as_str(),
                        get_dependency_version("chrono")?.as_str()
                    ).as_str(),
                    false,
                ),
            ])
            .thumbnail(developer.avatar_url().unwrap_or_default())
            .color(0x00ff00)
            .footer(serenity::CreateEmbedFooter::new(format!("Version {} | {}", env!("CARGO_PKG_VERSION"), timestamp)).icon_url(bot_avatar));

        ctx.send(CreateReply::default().embed(embed)).await?;
    } else {
        ctx.say("Error while trying to send embed").await?;
    }

    Ok(())
}

fn get_rust_version() -> Result<String, Error> {
    let output = std::process::Command::new("rustc").arg("--version").output()?;
    let full_version = String::from_utf8(output.stdout)?;
    let version_parts: Vec<&str> = full_version.split_whitespace().collect();
    Ok(version_parts.get(1).map_or(full_version.trim().to_string(), |s| s.to_string()))
}

fn get_dependency_version(package_name: &str) -> Result<String, Error> {
    let contents = fs::read_to_string("Cargo.lock")?;
    let value: Value = toml::from_str(&contents)?;
    let packages = value["package"].as_array().ok_or("No packages found")?;
    packages
        .iter()
        .find(|package| package["name"].as_str() == Some(package_name))
        .map(|package| package["version"].as_str().unwrap_or("Unknown").to_string())
        .ok_or_else(|| format!("Package {} not found", package_name).into())
}