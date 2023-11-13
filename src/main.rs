use anyhow::{bail, Result};
use clap::Parser;
use lazy_static::lazy_static;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use std::{
    env,
    fs::{self, File},
    io,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
use tempdir::TempDir;
use zip::ZipArchive;

mod firechicken;
mod templates;

use firechicken::FireChicken;

lazy_static! {
    pub static ref GIT_SHA: String = {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .expect("Failed to eecute git command");
        String::from_utf8(output.stdout).expect("Failed to parse git output")
    };
    pub static ref GIT_SHA_SHORT: String = GIT_SHA.chars().take(7).collect();
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

    // Get hash for style.css
    let css_hash: String = blake3::hash(include_bytes!("../static/style.css"))
        .to_string()
        .chars()
        .take(16)
        .collect();

    // Recreate dir
    fs::remove_dir_all("dist").ok();
    fs::create_dir_all("dist")?;

    // Copy static files
    copy_dir("static", "dist/")?;

    // Create /
    fs::write(
        "dist/index.html",
        templates::index(&fire_chicken, &css_hash)?.into_string(),
    )?;

    // Create /colophon
    fs::create_dir_all("dist/colophon")?;
    fs::write(
        "dist/colophon/index.html",
        templates::colophon(&css_hash)?.into_string(),
    )?;

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
