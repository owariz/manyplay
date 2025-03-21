use glob::glob;
use crate::commands;
use lavalink_rs::prelude::LavalinkClient;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn load_commands() -> Vec<poise::Command<LavalinkClient, Error>> {
    let mut commands = vec![];

    for entry in glob("src/commands/*.rs").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            let module_name = path.file_stem().unwrap().to_str().unwrap();

            if module_name == "mod" {
                continue;
            }

            if module_name == "ping" {
                commands.push(commands::ping::ping());
                println!("Loaded command: ping");
            } else if module_name == "botinfo" {
                commands.push(commands::botinfo::botinfo());
                println!("Loaded command: botinfo");
            } else if module_name == "join" {
                commands.push(commands::join::join());
                println!("Loaded command: join");
            } else if module_name == "play" {
                commands.push(commands::play::play());
                println!("Loaded command: play");
            } else {
                println!("Unknown module: {}", module_name);
            }
        } else if let Err(e) = entry {
            println!("{:?}", e);
        }
    }

    commands
}