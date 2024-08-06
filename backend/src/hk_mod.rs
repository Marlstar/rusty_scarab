#[derive(Debug, Clone)]
pub struct HkMod {
    pub name: String,
    pub description: String,
    pub version: [usize;4],
    pub link: String,
    pub hash: String,
    pub repository: String,
    pub dependencies: Vec<String>,
    pub integrations: Vec<String>,
    pub tags: Option<Vec<String>>,
    pub authors: Vec<String>,
}
impl HkMod {
    pub fn get_mod_file(&self) {
        return;
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
    pub dependencies: Option<Vec<String>>,
    pub integrations: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
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
                Some(a) => a.clone(),
                None => return None
            },
            repository: match &self.repository {
                Some(a) => a.clone(),
                None => return None
            },
            dependencies: match &self.dependencies {
                Some(a) => a.clone(),
                None => return None
            },
            integrations: match &self.integrations {
                Some(a) => a.clone(),
                None => return None
            },
            tags: self.tags.clone(),
            authors: match &self.authors {
                Some(a) => a.clone(),
                None => return None
            },
        };
        return Some(m);
    }
}
