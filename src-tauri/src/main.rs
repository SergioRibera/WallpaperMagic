use std::env;
mod app;
mod utils;
mod models;

use app::launch_app_gui;
use utils::{ read_args, load_data_app, save_config, oneinstance::create_app_lock };
use models::ApplicationDataConfig;


fn main() {
    let mut app_data: ApplicationDataConfig = ApplicationDataConfig::default();
    let args_matches = read_args(env::args().collect());
    load_data_app(&mut app_data, args_matches.clone());
    println!("{:?}", app_data);

    if env::args().len() > 1 {
        if let Some(cfg) = args_matches.value_of("config") {
            println!("The config file is: {:?}", cfg);
        }
        if args_matches.is_present("next") {
            println!("The Next Wallpaper");
        }
        save_config(app_data);
        // let oneinstance = create_app_lock(app_data.port());
    }
    if !args_matches.is_present("nogui") {
        launch_app_gui();
    }
}
