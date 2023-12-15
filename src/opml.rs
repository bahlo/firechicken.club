use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use url::Url;

use crate::firechicken::FireChicken;

const XML_DECLARATION: &'static str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

#[derive(Serialize)]
#[serde(rename = "opml")]
struct Opml {
    #[serde(rename = "@version")]
    version: String,
    head: Head,
    body: Body,
}

#[derive(Serialize)]
#[serde(rename = "head")]
struct Head {
    title: String,
    #[serde(with = "rfc_822")]
    date_created: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename = "body")]
struct Body {
    outline: Vec<Outline>,
}

#[derive(Serialize)]
#[serde(rename = "outline")]
struct Outline {
    #[serde(rename = "@text")]
    text: String,
    #[serde(rename = "@title")]
    title: String,
    #[serde(rename = "@type")]
    typ: String,
    #[serde(rename = "@xmlUrl")]
    xml_url: Url,
    #[serde(rename = "@htmlUrl")]
    html_url: Url,
}

pub fn render(fire_chicken: &FireChicken) -> Result<String> {
    let opml = Opml {
        version: "1.0".to_string(),
        head: Head {
            title: "RSS Feeds for all Fire Chicken Webring members".to_string(),
            date_created: Utc::now(),
        },
        body: Body {
            outline: fire_chicken
                .members
                .iter()
                .filter(|m| !m.invalid)
                .map(|member| {
                    member.rss_feeds.iter().map(|feed| Outline {
                        text: feed.title.clone().unwrap_or(member.name.clone()),
                        title: feed.title.clone().unwrap_or(member.name.clone()),
                        typ: "rss".to_string(),
                        xml_url: feed.xml_url.clone(),
                        html_url: feed.html_url.clone().unwrap_or(member.url.clone()),
                    })
                })
                .flatten()
                .collect(),
        },
    };

    let mut xml = String::new();
    xml.push_str(XML_DECLARATION);
    xml.push_str(&quick_xml::se::to_string(&opml)?);

    Ok(xml)
}

mod rfc_822 {
    use chrono::{DateTime, Utc};
    use serde::{self, Serializer};

    const FORMAT: &str = "%a, %d %b %Y %H:%M:%S %z";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}
