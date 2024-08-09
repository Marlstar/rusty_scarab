use inline_colorization::*;
use tokio::io::AsyncWriteExt;
use std::path::Path;

pub async fn download(url: &str, write_path: &str, mod_name: &str, mod_version: &[usize;4]) -> Result<String, Box<dyn std::error::Error>> {
    let _tempdir = tempfile::tempdir()?;
    let tempdir = _tempdir.path();

    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;
    println!("{} bytes", body.len());

    let fname = url.split("/").last().unwrap();
    let path = tempdir.join(fname);

    println!("{color_green}Downloading file: {fname}{color_reset}");
    println!(" -> from url: {color_blue}{url}{color_reset}");
    println!(" -> to temporary directory: {color_blue}{}{color_reset}", tempdir.display());
    println!(" -> writing to directory: {color_blue}{write_path}{color_reset}");
    println!("{color_green}Mod download info{color_reset}");

    let mut file = tokio::fs::File::create(path.clone()).await?;
    file.write_all(&body).await?;
    
    let ext = path.extension().unwrap();
    let cpath = match ext.to_str().expect("failed to extract file extension") {
        "dll" => {
            println!(" -> Found dll: {color_blue}{}{color_reset}", path.display());
            path.clone()
        },
        "zip" => {
            println!(" -> Found zip: {color_blue}{}{color_reset}", path.display());
            extract_dll_from_zip(&path, &tempdir)
        },
        _ => panic!("unsupported mod filetype!")
    };

    println!(" -> Moving dll to mods folder");

    let final_path = format!(
        "{}\\{}-{}.{}.{}.{}.dll",
        crate::ScarabDir::MOD.dir(),
        mod_name,
        mod_version[0], mod_version[1], mod_version[2], mod_version[3], 
    );
    move_file(&cpath, final_path.as_str()).await;

    Ok(final_path)
}

async fn move_file(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    let _ = tokio::fs::rename(from, to).await;
}

fn extract_dll_from_zip(zip_path: &Path, tempdir: &Path) -> std::path::PathBuf {
    use std::fs::File;

    let zip_file = File::open(zip_path).expect("failed to open zip file");
    let mut archive = zip::read::ZipArchive::new(zip_file).expect("failed to create zip reader");

    for i in 0..archive.len() {
        let mut f = archive.by_index(i).unwrap();

        if f.name().ends_with(".dll") {
            let file_name = f.name();
            println!(" -> Found dll in zip: {:?}", file_name);
            let mut out = File::create(format!("{}\\{}", tempdir.display(), f.name())).unwrap();

            let _ = std::io::copy(&mut f, &mut out);
            return format!("{}\\{}", tempdir.display(), f.name()).into();
        }
    }
    panic!("no dll found");
}
