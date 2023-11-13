use anyhow::{anyhow, bail, Result};
use chrono::NaiveDate;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{fs, path::Path};
use url::Url;

#[derive(Debug, Deserialize)]
struct FireChicken {
    members: Vec<Member>,
}

impl FireChicken {
    fn random(&self) -> Option<&Member> {
        let mut rng = rand::thread_rng();
        self.members.choose(&mut rng)
    }

    fn prev(&self, slug: &str) -> Option<&Member> {
        let Some(index) = self.members.iter().position(|member| member.slug == slug) else {
            return None;
        };

        if index == 0 {
            self.members.last()
        } else {
            self.members.get(index - 1)
        }
    }

    fn next(&self, slug: &str) -> Option<&Member> {
        let Some(index) = self.members.iter().position(|member| member.slug == slug) else {
            return None;
        };

        if index == self.members.len() - 1 {
            self.members.first()
        } else {
            self.members.get(index + 1)
        }
    }
}

#[derive(Debug, Deserialize)]
struct Member {
    slug: String,
    url: Url,
    name: String,
    joined: NaiveDate,
}

fn main() -> Result<()> {
    let fire_chicken: FireChicken =
        toml::from_str(include_str!("../firechicken.toml")).expect("Invalid TOML");

    // Recreate dir
    fs::remove_dir_all("dist").ok();
    fs::create_dir_all("dist")?;

    // Copy static files
    copy_dir("static", "dist/")?;

    fs::write("dist/index.html", index(&fire_chicken)?.into_string())?;

    Ok(())
}

fn index(fire_chicken: &FireChicken) -> Result<Markup> {
    Ok(html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { "Fire Chicken Webring" }
                meta charset="utf-8";
                meta name="title" content="Fire Chicken Webring";
                meta name="description" content="An invite-only webring for personal websites.";
                meta name="author" content="Arne Bahlo";
                meta name="theme-color" content="color(srgb 0.9429 0.3521 0.1599)";
                meta name="viewport" content="width=device-width,initial-scale=1";
                meta property="og:type" content="website";
                meta property="og:url" content="firechicken.club";
                meta property="og:title" content="Fire Chicken Webring";
                meta property="og:description" content="An invite-only webring for personal websites.";
                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="manifest" href="/site.webmanifest";
                link rel="stylesheet" href="/style.css";
            }
            body {
                .sitewrapper.stack {
                    header.hero {
                        h1.hero__heading {
                            "Fire"
                            br;
                            "Chicken"
                            br;
                            "Webring"
                        }
                        .hero__fire_chicken {
                            img src="/fire-chicken.svg" alt="A chicken with sunglasses and a tail of fire";
                        }
                    }
                    main.stack {
                        p.description {
                            "An invite-only webring for personal websites."
                        }
                        div {
                            a href=(format!("/{}/prev", fire_chicken.members.first().ok_or(anyhow!("Failed to get first member"))?.slug)) { "←" }
                            " "
                            a href="https://firechicken.club" { "🔥🐓" }
                            " "
                            a href="/random" { "Random" }
                            " "
                            a href=(format!("/{}/next", fire_chicken.members.last().ok_or(anyhow!("Failed to get last member"))?.slug)) { "→" }
                        }
                        table.members {
                            thead {
                                th { "Slug" }
                                th { "Name" }
                                th { "Url" }
                            }
                            tbody {
                                @for member in fire_chicken.members.iter() {
                                    tr {
                                        td { (member.slug) }
                                        td { (member.name) }
                                        td {
                                            a href=(member.url) { (member.url.host().ok_or(anyhow!("Failed to get host from {}", member.url))?) }
                                        }
                                    }
                                }
                            }
                        }
                        h2 { "FAQ" }
                        section.stack-small {
                            details {
                                summary { "What is a webring?" }

                                p {
                                    "A webring is a collection of website, usually grouped by a topic, so people that want to find websites with similar content can find those easily. They were popular in the 90s due to bad search engines. Now they’re "
                                    em { "niche" }
                                    "."
                                }
                            }
                            details {
                                summary { "How do I join?" }

                                p {
                                    "If a friend of yours is in the webring, ask them to send me an email with your email address and your website."
                                }
                            }
                        }
                    }
                    footer {
                        span { (PreEscaped("&copy;")) " 2023 by Arne Bahlo" }
                    }
                }
            }
        }
    })
}

fn copy_dir<F, T>(from: F, to: T) -> Result<()>
where
    F: AsRef<Path> + Send + Sync,
    T: AsRef<Path> + Send,
{
    // TODO: Turn this into functional code
    let mut dir = fs::read_dir(&from)?;
    while let Some(item) = dir.next().transpose()? {
        let file_name = item.file_name();

        let file_name_str = file_name.to_string_lossy();
        if file_name_str.starts_with('.') && file_name_str != ".well-known" {
            continue;
        }

        let new_path = to.as_ref().join(file_name);
        if new_path.exists() {
            bail!("File or directory already exists: {:?}", new_path)
        }

        if item.path().is_dir() {
            fs::create_dir(&new_path)?;
            copy_dir(item.path(), &new_path)?;
        } else {
            let path = item.path();
            fs::copy(path, new_path)?;
        }
    }

    Ok(())
}
