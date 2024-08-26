use std::fmt::format;

use crate::ModDatabase;
use crate::ScarabDir;
use crate::ModVersion;

pub struct ModManager {
    db: ModDatabase
}
impl ModManager {
    pub async fn install_mod(&mut self, mod_name: &String) {
        let mod_base_dir = ScarabDir::MOD.dir();
        let mod_dir = format!("{mod_base_dir}/{mod_name}");

        let mod_info = self.db.mod_info(mod_name).await.unwrap();

        let installed_version = Self::mod_installed(mod_name);

        // First check if the mod is installed
        if let Some(ver) = installed_version {
            // Check if update is available
            if crate::newer_version_than(&mod_info.version, &ver) {
                self.uninstall_mod_file(mod_name).await;
                self.db.download_mod(mod_name).await;
            }
        }
        else { // Not installed
            self.db.download_mod(mod_name).await;
        }
    }

    pub async fn uninstall_mod_file(&mut self, mod_name: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mod_base_dir = ScarabDir::MOD.dir();
        let mod_dir = format!("{mod_base_dir}/{mod_name}");

        let mut read_dir = tokio::fs::read_dir(mod_dir).await?;
        while let Ok(Some(item)) = read_dir.next_entry().await {
            if item.path().extension() == Some(std::ffi::OsStr::new("dll")) {
                tokio::fs::remove_file(item.path());
            }
        }

        Ok(())
    }

    pub async fn uninstall_mod_full(&mut self, mod_name: &String) -> tokio::io::Result<()> {
        let mod_base_dir = ScarabDir::MOD.dir();
        let mod_dir = format!("{mod_base_dir}/{mod_name}");

        return tokio::fs::remove_dir_all(mod_dir).await;
    }

    pub fn mod_installed(mod_name: &String) -> Option<ModVersion> {
        let mod_base_dir = ScarabDir::MOD.dir();
        let mod_dir = format!("{mod_base_dir}/{mod_name}");
        
        if !std::path::Path::new(&mod_dir).exists() {
            return None;
        }

        for file in std::fs::read_dir(&mod_dir).expect("Failed to open mod directory to read") {
            let file = &file.unwrap().path();
            if !file.is_file() { continue };
            if file.extension() != Some(std::ffi::OsStr::new("dll")) { continue };

            let name = file.file_stem().unwrap();
            let version_vec: Vec<usize> = name.to_str().unwrap() // File name not including extension
                .split('-').nth_back(0).unwrap() // Raw version string
                .split('.').map(|n| n.parse::<usize>().unwrap()).collect(); // Version numbers
            let version: ModVersion = [version_vec[0],version_vec[1],version_vec[2],version_vec[3]];
            return Some(version);
        }
        todo!();
    }
}
