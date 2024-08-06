use crate::{HkMod, ModConstuctor};
use std::collections::HashMap;

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
        // let raw_xml = Self::get_modlist_xml().await?;
        let raw_xml = TEST_MOD_XML_ELEMENT.to_string();
        let xml = Self::parse_modlist_xml(&raw_xml).await?;
        
        for m in xml.descendants().filter(|n| n.tag_name().name() == "Manifest") {
            // Per mod
            let mut cm = ModConstuctor::new();
            for c in m.descendants() {
                let ct: Option<String> = match c.text() {
                    Some(a) => Some(a.trim().to_string()),
                    None => None
                };
                println!("Current text: {:?}", ct);
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

                    _ => ()
                }

            }
            println!("{:#?}", cm);
            todo!()
        }

        todo!();
    }

    async fn get_modlist_xml() -> Result<String, reqwest::Error> {
        let response_text = reqwest::get(MOD_LIST_XML).await?.text().await?;
        return Ok(response_text);
    }
    async fn parse_modlist_xml(raw_xml: &String) -> Result<roxmltree::Document, roxmltree::Error> {
        return roxmltree::Document::parse(raw_xml);
    }
}
