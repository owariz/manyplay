use glob::glob;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn load_commands() -> Vec<poise::Command<(), Error>> {
    let mut commands = vec![];

    for entry in glob("src/commands/*.rs").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            let module_name = path.file_stem().unwrap().to_str().unwrap();

            if module_name == "mod" {
                continue;
            }

            if module_name == "ping" {
                commands.push(crate::commands::ping::ping());
                println!("Loaded command: ping");
            } else if module_name == "botinfo" {
                commands.push(crate::commands::botinfo::botinfo());
                println!("Loaded command: botinfo");
            } else if module_name == "join" {
                commands.push(crate::commands::join::join());
                println!("Loaded command: join");
            } else {
                println!("Unknown module: {}", module_name);
            }
        } else if let Err(e) = entry {
            println!("{:?}", e);
        }
    }

    commands
}