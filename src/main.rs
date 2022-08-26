use std::{env, fs, path::Path, process::exit};
static XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1
        || args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
    {
        println!("Provide file name(s) or pass -a/--about to see info about used libraries");
        exit(1);
    }

    let mut files = vec![];
    let mut obs_json_path = String::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-a" | "--about" => {
                let tpty = include_str!("../thirdparty/THIRDPARTY");
                println!("{}", tpty);
                return;
            }
            "-p" | "--path" => {
                obs_json_path = args[i + 1].clone();
                i = i + 2;
            }
            _ => {
                let file_path = Path::new(args.get(i).unwrap())
                    .canonicalize()
                    .unwrap_or_else(|_| panic!("Couldn't deref path {}", stringify!(i)));
                files.push(file_path);
                i = i + 1;
            }
        }
    }

    let config_dir = env::var(XDG_CONFIG_HOME);

    if obs_json_path.is_empty() {
        match config_dir {
            Ok(i) => obs_json_path = format!("{}/obsidian/obsidian.json", i),
            Err(_) => panic!(
                "Config dir not found at {}. Please set a custom path to your obsidian.json with -p/--path", XDG_CONFIG_HOME,
            )
        }
    }

    let obsidian_json_contents = fs::read_to_string(obs_json_path.as_str())
        .unwrap_or_else(|_| panic!("Couldn't read {}", obs_json_path));
    let obs_json_parsed = json::parse(obsidian_json_contents.as_str())
        .unwrap_or_else(|_| panic!("Couldn't parse {}", obs_json_path));

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
                    .unwrap_or_else(|_| panic!("Could not open obsidian link {}", new_path));
                break;
            } else {
                open::that(file.to_str().expect("Couldn't convert found path to str"))
                    .expect("Could not open using text editor");
            }
        }
    }
}
