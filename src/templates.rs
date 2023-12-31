use crate::{FireChicken, GIT_SHA, GIT_SHA_SHORT};
use anyhow::{anyhow, Result};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use url::Url;

#[derive(Debug)]
pub struct Head<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub url: Url,
    pub css_hash: &'a str,
}

fn layout(head: Head, contents: Markup) -> Result<Markup> {
    Ok(html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { "Fire Chicken Webring" }
                meta charset="utf-8";
                meta name="title" content=(head.title);
                meta name="description" content=(head.description);
                meta name="author" content="Arne Bahlo";
                meta name="theme-color" content="color(srgb 0.9429 0.3521 0.1599)";
                meta name="viewport" content="width=device-width,initial-scale=1";
                meta property="og:type" content="website";
                meta property="og:url" content=(head.url.as_str());
                meta property="og:title" content=(head.title);
                meta property="og:description" content=(head.description);
                meta property="og:image" content="https://firechicken.club/og-image.png";
                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="manifest" href="/site.webmanifest";
                link rel="stylesheet" href=(format!("/style.css?hash={}", head.css_hash));
            }
            body {
                .sitewrapper.stack {
                    (contents)
                    footer {
                        span {
                            (PreEscaped("&copy;")) " 2023 "
                            a href="https://arne.me" { "Arne Bahlo" }
                            (PreEscaped(" &middot; "))
                            a rel="me" href="https://spezi.social/@firechicken" { "Mastodon" }
                            (PreEscaped(" &middot; "))
                            "Commit "
                            a href=(format!("https://github.com/bahlo/firechicken.club/commit/{}", *GIT_SHA)) { (*GIT_SHA_SHORT) };
                            (PreEscaped(" &middot; "))
                            a href="/colophon" { "Colophon" };
                        }
                    }
                }
            }
        }
    })
}

pub fn index(fire_chicken: &FireChicken, css_hash: impl AsRef<str>) -> Result<Markup> {
    layout(
        Head {
            title: "Fire Chicken Webring",
            description: "An invite-only webring for personal websites.",
            url: Url::parse("https://firechicken.club")?,
            css_hash: css_hash.as_ref(),
        },
        html! {
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
                    "This is what it looks like: "
                    a.no-underline href=(format!("/{}/prev", fire_chicken.members.first().ok_or(anyhow!("Failed to get first member"))?.slug)) { "←" }
                    (PreEscaped("&nbsp;"))
                    a href="https://firechicken.club" { (PreEscaped("Fire&nbsp;Chicken&nbsp;Webring")) }
                    (PreEscaped("&nbsp;"))
                    a.no-underline href=(format!("/{}/next", fire_chicken.members.last().ok_or(anyhow!("Failed to get last member"))?.slug)) { "→" }

                    " or "

                    a.no-underline href=(format!("/{}/prev", fire_chicken.members.first().ok_or(anyhow!("Failed to get first member"))?.slug)) { "←" }
                    (PreEscaped("&nbsp;"))
                    a.no-underline href="https://firechicken.club" { (PreEscaped("🔥&#8288;🐓")) }
                    (PreEscaped("&nbsp;"))
                    a.no-underline href=(format!("/{}/next", fire_chicken.members.last().ok_or(anyhow!("Failed to get last member"))?.slug)) { "→" }
                }
                table.members {
                    thead {
                        th { "Slug" }
                        th { "Name" }
                        th { "Url" }
                    }
                    tbody {
                        @for member in fire_chicken.members.iter() {
                            tr class=(if member.invalid { "line-through" } else { "" }) {
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
                        summary { "Can I subscribe to all websites at once?" }
                        p {
                            "Yes, there's an "
                            a href="/opml.xml" { "OPML file" }
                            " you can import into your RSS reader to subscribe to all sites at once."
                        }
                    }
                    details {
                        summary { "How do I join?" }

                        .stack-small {
                            p {
                                "If a friend of yours is in the webring, ask them to send me an email with your email address and your website. Once you're in, add the following snippet somewhere, replacing "
                                code { ":slug" }
                                " with your slug:"
                            }

                            pre {
                                code {
                                    r#"<a href="https://firechicken.club/:slug/prev">←</a>
<a href="https://firechicken.club">Fire Chicken Webring</a>
<a href="https://firechicken.club/:slug/next">→</a>"#
                                }
                            }
                        }
                    }
                    details {
                        summary { "Why are some members crossed out?" }
                        p {
                            "When the links are missing or the site is down for a longer period of time, the member is marked as invalid and skipped in the ring. If you're a member and you think you're marked as invalid by mistake, please contact me."
                        }
                    }
                }
            }
        },
    )
}

pub fn colophon(css_hash: impl AsRef<str>) -> Result<Markup> {
    layout(
        Head {
            title: "Colophon",
            description: "The colophon for the Fire Chicken Webring.",
            url: Url::parse("https://firechicken.club/colophon")?,
            css_hash: css_hash.as_ref(),
        },
        html! {
            a href="/" { "← Index" }
            h2 { "Colophon" }
            p {
                "This website was first published on November 13th, 2023 near "
                a href="https://frankfurt.de" { "Frankfurt, Germany" }
                ". It's developed on a 2021 MacBook Pro using a custom Rust application and hosted on "
                a href="https://netlify.com" { "Netlify" }
                ". The code is hosted on "
                a href="https://github.com/bahlo/firechicken.club" { "GitHub" }
                "."
            }
            p {
                "Testing was conducted in the latest versions of Edge, Chrome, Firefox, and Safari. Any issue you encounter on this website can be submitted as "
                a href="https://github.com/bahlo/firechicken.club/issues/new" { "GitHub issues" }
                "."
            }
            p {
                "The logo was made for this website by "
                a href="https://www.instagram.com/ekkapranova/" { "Ekaterina Kapranova" }
                "."
            }
            p {
                "The font in the header is "
                a.obviously-condensed href="https://ohnotype.co/fonts/obviously" { "Obviously Condensed" }
                " by the Ohno Type Company."
            }
        },
    )
}

pub fn not_found(css_hash: impl AsRef<str>) -> Result<Markup> {
    layout(
        Head {
            title: "Not Found",
            description: "This page could not be found.",
            url: Url::parse("https://firechicken.club/404")?,
            css_hash: css_hash.as_ref(),
        },
        html! {
            a href="/" { "← Index" }
            h2 { "404 — Not Found" }
            p {
                "The page you were looking for was not found."
            }
        },
    )
}
