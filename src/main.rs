use backend::{ModDatabase, ScarabDir};


#[tokio::main]
async fn main() {
    // Clear terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // App
    backend::make_dirs();
    backend::clean_dir(ScarabDir::MOD);
    backend::clean_dir(ScarabDir::DOWNLOAD);
    let mut md = ModDatabase::new();
    let _ = md.get_mods().await;
    md.download_mod(&String::from("Benchwarp")).await;
}
