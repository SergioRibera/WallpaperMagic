use clap::{Arg, ArgMatches, App, AppSettings};
use dirs::config_dir;

use std::fs::{File, OpenOptions, create_dir_all};
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

use crate::models::{ ApplicationDataConfig, ApplicationFiles, RandomizeType, BackgroundType };

pub mod oneinstance;

pub fn read_args(args: Vec<String>) -> ArgMatches {
    let path_conf_file = config_dir().unwrap().join("wallpapermagic/config.json");
    let app = App::new("Wallpaper Magic")
        .version("0.1.0")
        .author("Sergio Ribera. <contact@sergioribera.com>")
        .about("Incredible wallpaper manipulation with amazing features")
        .setting(AppSettings::ColorAlways)
        .arg(Arg::new("config")
             .short('c')
             .long("config")
             .value_name("FILE")
             .about("Sets a custom config file")
             .default_value(path_conf_file.as_os_str().to_str().unwrap())
             .takes_value(true))
        .arg(Arg::new("port")
             .long("port")
             .value_name("PORT")
             //.default_value("6789")
             .about("This is the port of the unique instance")
             .takes_value(true))
        .arg(Arg::new("next")
             .long("next")
             .short('n')
             .about("Change the next wallpaper (Random if it's selected true)"))
        .arg(Arg::new("preview")
             .long("preview")
             .short('p')
             .about("Change the preview wallpaper"))
        .arg(Arg::new("random")
             .long("random")
             .short('r')
             .about("Enable randomize order of images selecteds")
             .possible_values(&["NextAsRandom", "ShuffleRandom", "Disabled"])
             .takes_value(true))
        .arg(Arg::new("background")
             .long("background-type")
             .short('b')
             .about("This define type for the background")
             .possible_values(&["Mix", "OnlyImg", "OnlyAnimated"])
             .takes_value(true))
        .arg(Arg::new("nogui")
             .long("nogui")
             .about("Not show Interface"))
        .arg(Arg::new("folders")
             .long("folders")
             .short('d')
             .about("All directories contain images/video/gif")
             .multiple(true)
             .min_values(1)
             .takes_value(true))
        .arg(Arg::new("files")
             .long("files")
             .short('f')
             .about("Files to Wallpaper images/video/gif")
             .multiple(true)
             .min_values(1)
             .takes_value(true))
        .arg(Arg::new("v")
             .short('v')
             .takes_value(true)
             .about("Enable verbosity"));
    if args.len() > 0 {
        return app.get_matches_from(args);
    }
    app.get_matches()
}

pub fn load_data_app(data: &mut ApplicationDataConfig, args_matches: ArgMatches) {
    let config_file: PathBuf = config_dir().unwrap().join("wallpapermagic/config.json");
    let mut config_file_path: &str = config_file.to_str().unwrap();
    if let Some(cfg_arg) = args_matches.value_of("config") {
        config_file_path = &cfg_arg;
    }
    let mut data_loaded: ApplicationDataConfig = load_config(&config_file_path);
    data_loaded.set_wallpapers_file(ApplicationFiles::default());
    data_loaded.set_randomize(RandomizeType::ShuffleRandom.to_string());
    data_loaded.set_bg_type(BackgroundType::Mix.to_string());
    data_loaded.set_enable_sound(false);

    if let Some(port) = args_matches.value_of("port") {
        data_loaded.set_port(port.parse::<u16>().unwrap());
    }
    if let Some(rand) = args_matches.value_of("random") {
        data_loaded.set_randomize(rand.parse().unwrap());
    }
    if let Some(back) = args_matches.value_of("background") {
        data_loaded.set_bg_type(back.parse().unwrap());
    }
    if let Some(folders) = args_matches.values_of("folders") {
        let mut folders_arr: Vec<String> = folders.map(|f| f.to_string()).collect::<Vec<String>>();
        data_loaded.wallpapers_file_mut().directories_mut().append(&mut folders_arr);
    }
    if let Some(files) = args_matches.values_of("files") {
        let mut files_arr: Vec<String> = files.map(|f| f.to_string()).collect::<Vec<String>>();
        data_loaded.wallpapers_file_mut().directories_mut().append(&mut files_arr);
    }

    *data = data_loaded.clone();
}

fn load_config (config_path_str: &str) -> ApplicationDataConfig {
    let mut config_file_path: PathBuf = PathBuf::new();
    config_file_path.push(Path::new(&config_path_str));

    let config_file_display = config_file_path.display();
    create_dir_all(config_file_path.parent().unwrap()).unwrap(); // Create directory if not exists
    let mut file: File;
    if config_file_path.exists() {
        file = match File::open(&config_file_path) {
            Err(why) => panic!("Couldn't open {}: {}", config_file_display, why),
            Ok(file) => file,
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Err(why) => panic!("Couldn't read from {}: {}", config_file_display, why),
            Ok(_) => {
                let data: ApplicationDataConfig = serde_json::from_str(&raw).unwrap();
                println!("Successfully loaded from {}", config_file_display);
                data
            }
        }
    } else {
        println!("Config File not Exists");
        save_config(ApplicationDataConfig::default());
        ApplicationDataConfig::default()
    }
}

pub fn save_config (data: ApplicationDataConfig) {
    let config_file_path: PathBuf = config_dir().unwrap().join("wallpapermagic/config.json");
    let config_file_display = config_file_path.display();
    create_dir_all(config_file_path.parent().unwrap()).unwrap(); // Create directory if not exists
    let data_serialized = serde_json::to_string(&data).unwrap();
    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_file_path.to_str().unwrap())
        .unwrap();
    match file.write_all(data_serialized.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", config_file_display, why),
        Ok(_) => println!("Successfully wrote to {}", config_file_display)
    }
}
