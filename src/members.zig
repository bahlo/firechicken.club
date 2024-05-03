const std = @import("std");

pub const RssFeed = struct {
    title: ?[]const u8,
    xml_url: []const u8,
    html_url: ?[]const u8,
};

pub const Member = struct {
    slug: []const u8,
    name: []const u8,
    url: []const u8,
    host: []const u8,
    joined: []const u8,
    invalid: bool,
    rss_feeds: []const RssFeed,
};

pub const members = [_]Member{ Member{ .slug = "arne", .name = "Arne Bahlo", .url = "https://arne.me", .host = "arne.me", .joined = "2023-11-12", .invalid = false, .rss_feeds = &[_]RssFeed{
    RssFeed{ .title = "Arne Bahlo", .xml_url = "https://arne.me/articles/atom.xml", .html_url = "https://arne.me/articles" },
    RssFeed{ .title = "Arne’s Weekly", .xml_url = "https://arne.me/weekly/atom.xml", .html_url = "https://arne.me/weekly" },
    RssFeed{ .title = "Arne Bahlo’s Book Reviews", .xml_url = "https://arne.me/book-reviews/feed.xml", .html_url = "https://arne.me/book-reviews" },
} }, Member{
    .slug = "ollie",
    .name = "ollie",
    .url = "https://flbn.sh",
    .host = "flbn.sh",
    .joined = "2023-11-13",
    .invalid = true,
    .rss_feeds = &[_]RssFeed{},
}, Member{ .slug = "lukas", .name = "Lukas Malkmus", .url = "https://lukasmalkmus.com", .host = "lukasmalkmus.com", .joined = "2023-11-13", .invalid = false, .rss_feeds = &[_]RssFeed{} }, Member{
    .slug = "pwa",
    .name = "Philipp Waldhauer",
    .url = "https://knuspermagier.de",
    .host = "knuspermagier.de",
    .joined = "2023-11-13",
    .invalid = false,
    .rss_feeds = &[_]RssFeed{
        RssFeed{ .title = undefined, .xml_url = "https://knuspermagier.de/feed", .html_url = undefined },
    },
}, Member{ .slug = "kotatsuyaki", .name = "kotatsuyaki", .url = "https://blog.kotatsu.dev", .host = "blog.kotatsu.dev", .joined = "2023-11-15", .invalid = false, .rss_feeds = &[_]RssFeed{
    RssFeed{ .title = undefined, .xml_url = "https://blog.kotatsu.dev/feed.xml", .html_url = undefined },
    RssFeed{ .title = "kotatsuyaki’s notes", .xml_url = "https://blog.kotatsu.dev/notes.xml", .html_url = "https://blog.kotatsu.dev/notes" },
} }, Member{ .slug = "cv", .name = "Christoph Voigt", .url = "https://christophvoigt.com", .host = "christophvoigt.com", .joined = "2023-11-15", .invalid = false, .rss_feeds = &[_]RssFeed{RssFeed{ .title = undefined, .xml_url = "https://christophvoigt.com/rss.xml", .html_url = undefined }} }, Member{ .slug = "igor", .name = "Igor Bedesqui", .url = "https://igorbedesqui.com", .host = "igorbedesqui.com", .joined = "2023-11-20", .invalid = false, .rss_feeds = &[_]RssFeed{} }, Member{ .slug = "hexedit", .name = "HexEdit Reality", .url = "https://hexeditreality.com/", .host = "hexeditreality.com", .joined = "2023-11-24", .invalid = false, .rss_feeds = &[_]RssFeed{RssFeed{ .title = undefined, .xml_url = "https://hexeditreality.com/index.xml", .html_url = undefined }} }, Member{ .slug = "stefan", .name = "Stefan Kühnel", .url = "https://stefankuehnel.com", .host = "stefankuehnel.com", .joined = "2023-11-27", .invalid = false, .rss_feeds = &[_]RssFeed{} }, Member{ .slug = "foreverliketh.is", .name = "Jayden Garridan Bridges", .url = "https://foreverliketh.is", .host = "foreverliketh.is", .joined = "2024-01-03", .invalid = false, .rss_feeds = &[_]RssFeed{RssFeed{ .title = undefined, .xml_url = "https://foreverliketh.is/blog/index.xml", .html_url = undefined }} }, Member{ .slug = "efe", .name = "İsmail Efe", .url = "https://ismailefe.org", .host = "ismailefe.org", .joined = "2024-02-21", .invalid = false, .rss_feeds = &[_]RssFeed{RssFeed{ .title = undefined, .xml_url = "https://ismailefe.org/feed.xml", .html_url = undefined }} }, Member{ .slug = "jan", .name = "Jan Früchtl", .url = "https://jan.work", .host = "jan.work", .joined = "2024-05-01", .invalid = false, .rss_feeds = &[_]RssFeed{RssFeed{ .title = undefined, .xml_url = "https://jan.work/feed", .html_url = undefined }} }, Member{ .slug = "laplab", .name = "Nikita Lapkov", .url = "https://laplab.me", .host = "laplab.me", .joined = "2024-05-03", .invalid = false, .rss_feeds = &[_]RssFeed{RssFeed{ .title = undefined, .xml_url = "https://laplab.me/posts/index.xml", .html_url = undefined }} } };
