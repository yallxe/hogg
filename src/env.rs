use directories::ProjectDirs;

static mut WARN_PRINTED: bool = false;

pub fn get_hogg_dir() -> String {
    match std::env::var("HOGG_CONFIG_DIR") {
        Ok(path) => path,
        Err(_) => {
            unsafe {
                if !WARN_PRINTED {
                    logs::info!(
                        "HOGG_CONFIG_DIR environment variable is not set, using default path"
                    );
                    WARN_PRINTED = true;
                }
            }
            ProjectDirs::from("", "", "Hogg").unwrap()
                .config_dir().to_str().unwrap().to_string()
        }
    }
}
