use anyhow::{anyhow, bail, Result};
use chrono::NaiveDate;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct FireChicken {
    pub members: Vec<Member>,
}

impl FireChicken {
    pub fn prev(&self, slug: impl AsRef<str>) -> Result<&Member> {
        let slug = slug.as_ref();

        let Some(index) = self.members.iter().position(|member| member.slug == slug) else {
            bail!("Member not found");
        };

        if index == 0 {
            self.members.last()
        } else {
            self.members.get(index - 1)
        }
        .ok_or_else(|| anyhow!("No members"))
    }

    pub fn next(&self, slug: impl AsRef<str>) -> Result<&Member> {
        let slug = slug.as_ref();

        let Some(index) = self.members.iter().position(|member| member.slug == slug) else {
            bail!("Member not found");
        };

        if index == self.members.len() - 1 {
            self.members.first()
        } else {
            self.members.get(index + 1)
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
}
