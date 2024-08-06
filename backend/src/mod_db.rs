use crate::{HkMod, ModConstuctor};
use std::collections::HashMap;

#[allow(dead_code)]
const MOD_LIST_XML: &str = "https://raw.githubusercontent.com/hk-modding/modlinks/main/ModLinks.xml";
const TEST_MOD_XML_ELEMENT: &str = "<Manifest>\n\
    <Name>ModCommon</Name>\n\
    <Description>A hollow knight mod that provides an API of helpers and other utilities for modding hollow knight.\n\
        Not recommended for use in new mods.</Description>\n\
    <Version>1.0.0.0</Version>\n\
    <Link SHA256=\"a4959501676033bb66136b466597f266794ef3627c92c4455c94b2a6450b69aa\">
        <![CDATA[https://github.com/HK-Modding-Preservation/ModCommon/releases/download/v1/ModCommon_1.5.dll]]>\n\
    </Link>\n\
    <Dependencies/>\n\
    <Repository>\n\
        <![CDATA[https://github.com/HK-Modding-Preservation/ModCommon]]>\n\
    </Repository>\n\
    <Tags>\n\
    <Tag>Library</Tag>\n\
    </Tags>\n\
    <Authors>\n\
    <Author>Kerr1291</Author>\n\
    <Author>Kerr1291_second</Author>\n\
    </Authors>\n\
    </Manifest>";


pub struct ModDatabase {
    mods: HashMap<String, HkMod>
}
impl ModDatabase {
    pub fn new() -> Self {
        return Self {
            mods: HashMap::new()
        }
    }
}

impl ModDatabase {
    pub async fn get_mods(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_xml = Self::get_modlist_xml().await?;
        // let raw_xml = TEST_MOD_XML_ELEMENT.to_string();
        let xml = Self::parse_modlist_xml(&raw_xml).await?;
        

        let mut mod_count = 0_usize;
        for m in xml.descendants().filter(|n| n.tag_name().name() == "Manifest") {
            // Per mod
            let mut cm = ModConstuctor::new();
            for c in m.descendants() {
                let ct: Option<String> = match c.text() {
                    Some(a) => Some(a.trim().to_string()),
                    None => None
                };

                match c.tag_name().name() {
                    "Name" => cm.name = ct,

                    "Description" => cm.description = ct,

                    "Version" => cm.version = Some(
                        match ct {
                            Some(t) => {
                                let v: Vec<usize> = t.split('.').map(|n| n.parse::<usize>().unwrap()).collect();
                                [v[0], v[1], v[2], v[3]]
                            },
                            None => continue
                        }
                    ),

                    "Link" => {
                        cm.link = ct;
                        cm.hash = Some(
                            c.attributes()
                                .filter(|a| a.name() == "SHA256")
                                .nth(0).unwrap()
                                .value()
                                .to_string()
                        );
                    },

                    "Repository" => cm.repository = ct,

                    "Authors" => {
                        let mut authors: Vec<String> = vec![];
                        for child in c.descendants().filter(|d| d.tag_name().name() == "Author") {
                            if let Some(a) = child.text() { authors.push(a.to_string()); }
                        }
                        cm.authors = Some(authors);
                    }

                    "Tags" => {
                        let mut tags: Vec<String> = vec![];
                        for child in c.descendants().filter(|d| d.tag_name().name() == "Tag") {
                            if let Some(a) = child.text() { tags.push(a.to_string()); }
                        };
                        if !tags.is_empty() {
                            cm.tags = Some(tags);
                        }
                    }

                    "Dependencies" => {
                        let mut dependencies: Vec<String> = vec![];
                        for child in c.descendants().filter(|d| d.tag_name().name() == "Dependency") {
                            if let Some(a) = child.text() { dependencies.push(a.to_string()); }
                        };
                        if !dependencies.is_empty() {
                            cm.dependencies = Some(dependencies);
                        }
                    }

                    "Integrations" => {
                        let mut integrations: Vec<String> = vec![];
                        for child in c.descendants().filter(|d| d.tag_name().name() == "Integration") {
                            if let Some(a) = child.text() { integrations.push(a.to_string()); }
                        };
                        if !integrations.is_empty() {
                            cm.integrations = Some(integrations);
                        }
                    }

                    _ => ()
                }

            }

            match cm.build() {
                Some(a) => {
                    mod_count += 1;
                    self.mods.insert(a.name.clone(), a);
                }
                None => {}
            }
        }

        println!("Parsed {mod_count} mods");
        Ok(())
    }

    async fn get_modlist_xml() -> Result<String, reqwest::Error> {
        let response_text = reqwest::get(MOD_LIST_XML).await?.text().await?;
        return Ok(response_text);
    }
    async fn parse_modlist_xml(raw_xml: &String) -> Result<roxmltree::Document, roxmltree::Error> {
        return roxmltree::Document::parse(raw_xml);
    }
}
