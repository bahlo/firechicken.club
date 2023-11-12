use axum::{routing::get, Router};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

#[derive(Debug, Deserialize)]
struct FireChicken {
    members: HashMap<String, Member>,
}

#[derive(Debug, Deserialize)]
struct Member {
    url: Url,
    name: String,
}

lazy_static! {
    static ref FIRE_CHICKEN: FireChicken =
        toml::from_str(include_str!("../firechicken.toml")).expect("Invalid TOML");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "firechicken_club=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(index))
        .fallback_service(ServeDir::new("static")); // TODO: Handle 404

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn index() -> Markup {
    html! {
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
                        p.description { "An invite-only webring for personal websites." }
                        h2 { "Members" }
                        ul.stack-small.members__list {
                            @for (slug, member) in FIRE_CHICKEN.members.iter() {
                                li.members__member {
                                    strong { (member.name) }
                                    br;
                                    a href=(member.url) { (member.url.host().unwrap()) }
                                }
                            }
                        }
                    }
                    footer {
                        span { (PreEscaped("&copy;")) " 2023 by Arne Bahlo" }
                        (PreEscaped(" &middot; "))
                        a href="/imprint" { "Imprint" }
                    }
                }
            }
        }
    }
}
