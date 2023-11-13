use anyhow::{anyhow, bail, Result};
use chrono::NaiveDate;
use clap::Parser;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
use tempdir::TempDir;
use url::Url;
use zip::ZipArchive;

#[derive(Debug, Deserialize)]
struct FireChicken {
    members: Vec<Member>,
}

impl FireChicken {
    fn prev(&self, slug: impl AsRef<str>) -> Result<&Member> {
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

    fn next(&self, slug: impl AsRef<str>) -> Result<&Member> {
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
struct Member {
    slug: String,
    url: Url,
    name: String,
    joined: NaiveDate,
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    #[clap(name = "build")]
    Build,
    #[clap(name = "watch")]
    Watch,
    #[clap(name = "download-fonts")]
    DownloadFonts,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build => build(),
        Commands::Watch => watch(),
        Commands::DownloadFonts => download_fonts(),
    }
}

fn build() -> Result<()> {
    let fire_chicken: FireChicken =
        toml::from_str(include_str!("../firechicken.toml")).expect("Invalid TOML");

    // Recreate dir
    fs::remove_dir_all("dist").ok();
    fs::create_dir_all("dist")?;

    // Copy static files
    copy_dir("static", "dist/")?;

    // Create /
    fs::write("dist/index.html", index(&fire_chicken)?.into_string())?;

    // Create redirects
    fs::write("dist/_redirects", redirects(&fire_chicken)?)?;

    Ok(())
}

fn watch() -> Result<()> {
    // Build on start
    build()?;

    let mut debouncer =
        new_debouncer(
            Duration::from_millis(500),
            |res: DebounceEventResult| match res {
                Ok(_event) => {
                    let mut child = match Command::new("cargo")
                        .arg("run")
                        .arg("build")
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                    {
                        Ok(child) => child,
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            return;
                        }
                    };

                    match child.wait() {
                        Ok(status) => {
                            if !status.success() {
                                eprintln!("Error: Received status {:?}", status);
                            }
                        }
                        Err(e) => eprintln!("Error: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Errro: {:?}", e),
            },
        )?;

    debouncer
        .watcher()
        .watch(Path::new("./src"), RecursiveMode::Recursive)?;
    debouncer
        .watcher()
        .watch(Path::new("./static"), RecursiveMode::Recursive)?;
    debouncer
        .watcher()
        .watch(Path::new("./Cargo.toml"), RecursiveMode::NonRecursive)?;
    debouncer
        .watcher()
        .watch(Path::new("./Cargo.lock"), RecursiveMode::NonRecursive)?;

    let dist = env::current_dir()?.join("dist");
    let server = file_serve::Server::new(&dist);
    println!("Running on http://{}", server.addr());
    println!("Hit CTRL-C to stop");
    server.serve()?;

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
                        span { (PreEscaped("&copy;")) " 2023 Arne Bahlo" }
                    }
                }
            }
        }
    })
}

fn redirects(fire_chicken: &FireChicken) -> Result<String> {
    let mut redirects = String::new();
    for member in fire_chicken.members.iter() {
        let prev = fire_chicken.prev(&member.slug)?;
        let next = fire_chicken.next(&member.slug)?;

        redirects.push_str(&format!(
            "{} {} 302\n",
            format!("/{}/prev", member.slug),
            prev.url,
        ));
        redirects.push_str(&format!(
            "{} {} 302\n",
            format!("/{}/next", member.slug),
            next.url,
        ));
    }
    Ok(redirects)
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

fn download_fonts() -> Result<()> {
    let zip_url = env::var("FONT_ZIP_URL")?;
    let destination = Path::new("./static/fonts");

    let response = ureq::get(&zip_url).call()?;
    let mut reader = response.into_reader();

    let temp_dir = TempDir::new("firechicken-club-fonts")?;
    let zip_path = temp_dir.path().join("fonts.zip");
    let mut temp_file = File::create(&zip_path)?;
    io::copy(&mut reader, &mut temp_file)?;

    let zip_file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => destination.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    temp_dir.close()?;
    Ok(())
}
