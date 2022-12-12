static mut WARN_PRINTED: bool = false;

pub fn get_hogg_dir() -> String {
    match std::env::var("HOGG_CONFIG_DIR") {
        Ok(path) => path,
        Err(_) => {
            unsafe {
                if !WARN_PRINTED {
                    logs::warn!("HOGG_CONFIG_DIR environment variable is not set, using default path");
                    WARN_PRINTED = true;
                }
            }
            ".hogg".to_string()
        }
    }
}