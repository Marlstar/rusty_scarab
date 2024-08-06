use backend::ModDatabase;


#[tokio::main]
async fn main() {
    let mut md = ModDatabase::new();
    let _ = md.get_mods().await;
}
