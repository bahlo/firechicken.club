use axum::{routing::get, Router};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="manifest" href="/site.webmanifest";
                link rel="stylesheet" href="/style.css";
            }
            body {
                .sitewrapper.stack {
                    header {
                        h1 { "Fire Chicken Webring" }
                        p { "An invite-only webring for personal websites." }
                    }
                    main {
                        section.members.stack-small {
                            h2 { "Members" }
                            ul.stack-small {
                                li {
                                    strong { "Arne Bahlo" }
                                    br;
                                    a href="https://arne.me" { "arne.me" }
                                }
                                li {
                                    strong { "Jan Früchtl" }
                                    br;
                                    a href="https://jan.work" { "jan.work" }
                                }
                            }
                        }
                        section.fire_chicken {
                            img src="/fire-chicken.svg" alt="A chicken with sunglasses and a tail of fire";
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
