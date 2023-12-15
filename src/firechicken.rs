use anyhow::{anyhow, bail, Result};
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct FireChicken {
    pub members: Vec<Member>,
}

impl FireChicken {
    pub fn prev_valid(&self, slug: impl AsRef<str>) -> Result<&Member> {
        let slug = slug.as_ref();

        let valid_members: Vec<&Member> = self.members.iter().filter(|m| !m.invalid).collect();

        let Some(index) = valid_members.iter().position(|member| member.slug == slug) else {
            bail!("Member not found");
        };

        if index == 0 {
            valid_members.last().map(|m| *m)
        } else {
            valid_members.get(index - 1).map(|m| *m)
        }
        .ok_or_else(|| anyhow!("No members"))
    }

    pub fn next_valid(&self, slug: impl AsRef<str>) -> Result<&Member> {
        let slug = slug.as_ref();

        let valid_members: Vec<&Member> = self.members.iter().filter(|m| !m.invalid).collect();

        let Some(index) = valid_members.iter().position(|member| member.slug == slug) else {
            bail!("Member not found");
        };

        if index == valid_members.len() - 1 {
            valid_members.first().map(|m| *m)
        } else {
            valid_members.get(index + 1).map(|m| *m)
        }
        .ok_or_else(|| anyhow!("No members"))
    }
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub slug: String,
    pub url: Url,
    pub name: String,
    #[allow(dead_code)]
    pub joined: NaiveDate,
    #[serde(default)]
    pub invalid: bool,
    #[serde(default)]
    pub rss_feeds: Vec<Url>,
}
