use directories::ProjectDirs;

static mut WARN_PRINTED: bool = false;

pub fn get_hogg_dir() -> String {
    match std::env::var("HOGG_CONFIG_DIR") {
        Ok(path) => path,
        Err(_) => {
            let default_path = ProjectDirs::from("", "", "Hogg").unwrap()
                .config_dir().to_str().unwrap().to_string();
            unsafe {
                if !WARN_PRINTED {
                    logs::info!(
                        "HOGG_CONFIG_DIR environment variable is not set, using {}",
                        default_path
                    );
                    WARN_PRINTED = true;
                }
            }
            default_path
        }
    }
}
