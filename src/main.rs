use std::{env, fs, path::Path, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1
        || args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
    {
        println!("Provide file name(s) or pass -a/--about to see info about used libraries");
        exit(1);
    }

    if args.contains(&String::from("-a")) || args.contains(&String::from("--about")) {
        let tpty = include_str!("../thirdparty/THIRDPARTY");
        println!("{}", tpty);
        return;
    }

    let mut files = vec![];

    for i in 1..args.len() {
        let file_path = Path::new(args.get(i).unwrap())
            .canonicalize()
            .expect("Couldn't deref path");
        files.push(file_path);
    }

    let config_dir = env::var("XDG_CONFIG_HOME").expect(
        "Config dir not found at XDG_CONFIG_HOME. Please set a custom path to your obsidian.json with -p/--path",
    );

    let obs_json_path = format!("{}/obsidian/obsidian.json", config_dir);
    let obsidian_json_contents = fs::read_to_string(obs_json_path.as_str())
        .expect(format!("Couldn't read {}", obs_json_path).as_str());
    let obs_json_parsed = json::parse(obsidian_json_contents.as_str())
        .expect(format!("Couldn't parse {}", obs_json_path).as_str());

    let vaults = obs_json_parsed["vaults"].entries();

    for f in vaults {
        let th_path = &f.1["path"];
        for file in &files {
            if file.starts_with(
                th_path
                    .as_str()
                    .expect("Could not get vault path as string"),
            ) {
                let urlencoded = urlencoding::encode(
                    file.to_str()
                        .expect("Couldn't convert found file path to str"),
                )
                .to_string();
                let new_path = format!("obsidian://open?path={}", urlencoded.as_str());
                open::that(new_path.clone())
                    .expect(format!("Could not open obsidian link {}", new_path).as_str());
                break;
            } else {
                open::that(file.to_str().expect("Couldn't convert found path to str"))
                    .expect("Could not open using text editor");
            }
        }
    }
}
