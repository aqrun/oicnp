use std::fs::File;
use std::io::Read;

use crate::utils::{AppConfig, get_config_file_path};

lazy_static! {
    pub static ref G: AppConfig = generate_app_config(get_config_file_path());
}

fn generate_app_config(
    file_path: String,
) -> AppConfig {
    let mut yaml_file = File::open(&file_path)
        .expect(&format!("File: [{}] not exist!", &file_path));
    let mut yml_data = String::new();

    yaml_file.read_to_string(&mut yml_data)
        .expect("Yaml 文件读取失败");

    let result: AppConfig = serde_yaml::from_str(&yml_data)
        .expect("Load app.yml failed!");

    if result.debug {
        println!("[oicnp] load config:{:?}", result);
        println!("[oicnp] ////////////////// Start on Debug Mode//////////////////");
    } else {
        println!("[oicnp] release_mode is enable!");
    }

    result
}
