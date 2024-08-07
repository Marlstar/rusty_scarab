#[derive(Debug, Clone)]
pub struct HkMod {
    pub name: String,
    pub description: String,
    pub version: [usize;4],
    pub link: String,
    pub hash: String,
    pub repository: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<String>,
    pub integrations: Vec<String>,
    pub tags: Vec<String>,
}
impl HkMod {
    pub async fn get_mod_file(&self) {
        // Get file
        use downloader::{Downloader, Download};
        let dl = Download::new(self.link.as_str());
        let mut downloader = Downloader::builder()
            .download_folder(std::path::Path::new(crate::directories::DOWNLOAD_DIR().as_str()))
            .build().unwrap();

        let dl_results = downloader.download(&[dl]);
        let result = dl_results.unwrap()
            .into_iter().nth(0)
            .unwrap().unwrap();
        let fname = result.file_name;

        let dl_hash = sha256::try_digest(fname.as_path()).unwrap().to_lowercase();
        if dl_hash != self.hash {
            panic!("hash assertion failed for file {:?}", fname);
        }

        // Make sure file is a dll, if not, unzip
        let dl_extension = fname.extension().unwrap().to_str().unwrap();

        match dl_extension {
            "dll" => {

            },
            // "zip" => extract_dll_from_zip(fname),
            _ => panic!("unexpected mod file format!")
        }
    }
}


#[derive(Debug, Clone)]
pub struct ModConstuctor {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<[usize;4]>,
    pub link: Option<String>,
    pub hash: Option<String>,
    pub repository: Option<String>,
    pub authors: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub integrations: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}
impl ModConstuctor {
    pub fn new() -> Self {
        return Self {
            name: None,
            description: None,
            version: None,
            link: None,
            hash: None,
            repository: None,
            dependencies: None,
            integrations: None,
            tags: None,
            authors: None
        }
    }

    pub fn build(&self) -> Option<HkMod> {
        let m = HkMod {
            name: match &self.name {
                Some(a) => a.clone(),
                None => return None
            },
            description: match &self.description {
                Some(a) => a.clone(),
                None => return None
            },
            version: match &self.version {
                Some(a) => a.clone(),
                None => return None
            },
            link: match &self.link {
                Some(a) => a.clone(),
                None => return None
            },
            hash: match &self.hash {
                Some(a) => a.to_lowercase(),
                None => return None
            },
            repository: match &self.repository {
                Some(a) => a.clone(),
                None => return None
            },
            dependencies: match &self.integrations {
                Some(a) => a.clone(),
                None => vec![]
            },
            integrations: match &self.integrations {
                Some(a) => a.clone(),
                None => vec![]
            },
            tags: match &self.tags {
                Some(a) => a.clone(),
                None => vec![]
            },
            authors: match &self.authors {
                Some(a) => a.clone(),
                None => vec![]
            },
        };
        return Some(m);
    }
}
