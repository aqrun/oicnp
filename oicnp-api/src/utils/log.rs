use oicnp_core::{
    prelude::{
        fast_log::{
            self,
            consts::LogSize,
            plugin::file_split::{Packer, RollingType},
            plugin::packer::{GZipPacker, LZ4Packer, LogPacker, ZipPacker},
            Config,
        },
        log::LevelFilter,
    },
    G,
};
use std::time::Duration;

pub fn init_log() {
    //create log dir
    let _ = std::fs::create_dir_all(&G.log_dir);

    // let packer = choose_packer(&G.log_pack_compress);
    //init fast log
    let fast_log_config = Config::new()
        .console()
        .level(str_to_log_level(&G.log_level))
        .file_split(
            &G.log_dir,
            str_to_temp_size(&G.log_temp_size),
            str_to_rolling(&G.log_rolling_type),
            LogPacker {},
        );
    let res = fast_log::init(fast_log_config);

    if let Err(err) = res {
        println!("Logger init error: {:?}", err);
    }

    if G.debug == false {
        println!("[OICNP] release_mode is up! [file_log] open,[console_log] disabled!");
    }
}

fn choose_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        "lz4" => Box::new(LZ4Packer {}),
        "zip" => Box::new(ZipPacker {}),
        "gzip" => Box::new(GZipPacker {}),
        _ => Box::new(LogPacker {}),
    }
}

fn str_to_temp_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(arg: &str) -> LevelFilter {
    return match arg {
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    };
}
