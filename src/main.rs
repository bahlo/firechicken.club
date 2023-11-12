use axum::{routing::get, Router};
use lazy_static::lazy_static;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde::Deserialize;
use std::collections::HashMap;
use tower_http::services::ServeDir;
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

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        .route("/", get(index))
        // .route("/random", get(random))
        // .route("/:slug/prev", get(prev))
        // .route("/:slug/next", get(next))
        .fallback_service(ServeDir::new("static"))
        .into()) // TODO: Handle 404
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
                        p.description {
                            "An invite-only webring for personal websites."
                        }
                        table.members {
                            thead {
                                th { "Slug" }
                                th { "Name" }
                                th { "Url" }
                            }
                            tbody {
                                @for (slug, member) in FIRE_CHICKEN.members.iter() {
                                    tr {
                                        td { (slug) }
                                        td { (member.name) }
                                        td {
                                            a href=(member.url) { (member.url.host().unwrap()) }
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
    }
}
