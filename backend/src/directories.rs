use std::env::var;

pub fn make_dirs() {
    std::fs::create_dir_all(BASE_DIR()).expect("failed to create base directory");
    std::fs::create_dir_all(MOD_DIR()).expect("failed to create mod directory");
    std::fs::create_dir_all(DOWNLOAD_DIR()).expect("failed to create download directory");
}

#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
pub fn BASE_DIR() -> String {
    return format!(
        "{}\\rusty_scarab",
        var("appdata").unwrap()
    );
}

#[allow(non_snake_case)]
pub fn MOD_DIR() -> String {
    return format!(
        "{}\\mods",
        BASE_DIR()
    );
}

#[allow(non_snake_case)]
pub fn DOWNLOAD_DIR() -> String {
    return format!(
        "{}\\downloads",
        BASE_DIR()
    );
}
