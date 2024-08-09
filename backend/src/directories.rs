use std::env::var;

pub fn make_dirs() {
    let dirs = { use ScarabDir::*; [BASE, MOD, DOWNLOAD] };
    for t in dirs {
        std::fs::create_dir_all(t.dir()).expect(format!("Failed to create {:?} directory at {:?}", t, t.dir()).as_str());
    }
}
pub fn clean_dir(dir: ScarabDir) {
    std::fs::remove_dir_all(dir.dir()).unwrap();
    std::fs::create_dir_all(dir.dir()).unwrap();
}


#[cfg(target_os = "windows")]
#[allow(non_snake_case)]
fn BASE_DIR() -> String {
    return format!(
        "{}\\rusty_scarab",
        var("appdata").unwrap()
    );
}


#[derive(Debug)]
pub enum ScarabDir {
    BASE,
    MOD,
    DOWNLOAD
}
impl ScarabDir {
    pub fn dir(&self) -> String {
        use ScarabDir::*;
        return match self {
            BASE => BASE_DIR(),
            MOD => atb("mods"),
            DOWNLOAD => atb("downloads"),
        }
    }

}

fn atb(i: &str) -> String {
    format!("{}\\{i}", BASE_DIR())
}



