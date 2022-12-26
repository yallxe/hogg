use directories::ProjectDirs;

static mut INFO_PRINTED: bool = false;

pub fn get_hogg_dir() -> String {
    let result = match std::env::var("HOGG_CONFIG_DIR") {
        Ok(path) => path,
        Err(_) => {
            let default_path = ProjectDirs::from("", "", "Hogg")
                .unwrap()
                .config_dir()
                .to_str()
                .unwrap()
                .to_string();
            default_path
        }
    };

    print_info(result.as_str());
    result
}

fn print_info(path: &str) {
    unsafe {
        if !INFO_PRINTED {
            logs::info!(
                "Using {} as a config directory. You can override this by setting HOGG_CONFIG_DIR environment variable",
                path
            );
            INFO_PRINTED = true;
        }
    }
}
