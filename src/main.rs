use backend::ModDatabase;


#[tokio::main]
async fn main() {
    backend::directories::make_dirs();
    let mut md = ModDatabase::new();
    let _ = md.get_mods().await;
    md.download_mod(&String::from("Benchwarp")).await;
}
