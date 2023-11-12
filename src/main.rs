use axum::{routing::get, Router};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const FIRE_CHICKEN: &str = include_str!("../static/fire-chicken.svg");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "firechicken_club=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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
                style { (PreEscaped(include_str!("../static/style.css"))) }
            }
            body {
                .sitewrapper {
                    h1 { "Fire Chicken Webring" }
                    p { "An invite-only webring for personal websites." }
                    main {
                        section.members {
                            h2 { "Members" }
                            ul {
                                li {
                                    a href="https://arne.me" { "Arne Bahlo" }
                                }
                            }
                        }
                        section.fire_chicken {
                            (PreEscaped(FIRE_CHICKEN))
                        }
                    }
                }
            }
        }
    }
}
