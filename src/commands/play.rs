use lavalink_rs::prelude::{GuildId as LavalinkGuildId, SearchEngines, TrackInQueue, TrackLoadData, LavalinkClient};
// use poise::serenity_prelude::{GuildId as SerenityGuildId};

use crate::commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, LavalinkClient, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search term or URL"]
    #[rest]
    term: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let guild_id_lavalink = LavalinkGuildId::from(guild_id.get());
    let has_joined = commands::join::join_helper(ctx).await?;
    let lava_client = ctx.data().clone();

    let Some(player) = lava_client.get_player_context(guild_id_lavalink) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let query = if let Some(term) = term {
        if term.starts_with("http") {
            term
        } else {
            SearchEngines::Deezer.to_query(&term)?
        }
    } else {
        if let Ok(player_data) = player.get_player().await {
            let queue = player.get_queue();
            if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
                player.skip()?;
            } else {
                ctx.say("The queue is empty.").await?;
            }
        }
        return Ok(());
    };

    let loaded_tracks = lava_client.load_tracks(guild_id_lavalink, &query).await?;
    let mut playlist_info = None;

    let mut tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }
        _ => {
            ctx.say("No tracks found.").await?;
            return Ok(());
        }
    };

    if let Some(info) = playlist_info {
        ctx.say(format!("Added playlist to queue: {}", info.name)).await?;
    } else {
        let track = &tracks[0].track;
        if let Some(uri) = &track.info.uri {
            ctx.say(format!(
                "Added to queue: [{} - {}](<{}>)",
                track.info.author, track.info.title, uri
            ))
            .await?;
        } else {
            ctx.say(format!(
                "Added to queue: {} - {}",
                track.info.author, track.info.title
            ))
            .await?;
        }
    }

    for track in &mut tracks {
        track.track.user_data = Some(serde_json::json!({"requester_id": ctx.author().id.get()}));
    }

    let queue = player.get_queue();
    queue.append(tracks.into())?;

    if has_joined {
        return Ok(());
    }

    if let Ok(player_data) = player.get_player().await {
        if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
            player.skip()?;
        }
    }

    Ok(())
}