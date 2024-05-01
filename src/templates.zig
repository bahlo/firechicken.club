const members = @import("members.zig").members;

pub const Templates = struct {
    git_sha: []const u8,
    css_hash: []const u8,
    date_created: []const u8,

    fn write_header(self: Templates, writer: anytype, title: []const u8, description: []const u8, url: []const u8) !void {
        try writer.print(
            \\<!DOCTYPE html>
            \\<html lang="en">
            \\  <head>
            \\    <title>{s}</title>
            \\    <meta charset="utf-8" />
            \\    <meta name="title" content="{s}" />
            \\    <meta name="description" content="{s}" />
            \\    <meta name="author" content="Arne Bahlo" />
            \\    <meta name="theme-color" content="color(srgb 0.9429 0.3521 0.1599)" />
            \\    <meta name="viewport" content="width=device-width,initial-scale=1" />
            \\    <meta property="og:type" content="website" />
            \\    <meta property="og:url" content="{s}" />
            \\    <meta property="og:title" content="{s}" />
            \\    <meta property="og:description" content="{s}" />
            \\    <meta property="og:image" content="https://firechicken.club/og-image.png" />
            \\    <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
            \\    <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
            \\    <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
            \\    <link rel="manifest" href="/site.webmanifest" />
            \\    <link rel="stylesheet" href="/style.css?hash={s}" />
            \\  </head>
            \\  <body>
            \\    <div class="sitewrapper stack">
            \\
        , .{ title, title, description, url, title, description, self.css_hash });
    }

    fn write_footer(self: Templates, writer: anytype) !void {
        try writer.print(
            \\
            \\    </div>
            \\    <footer>
            \\      &copy; 2023
            \\      <a href="https://arne.me">Arne Bahlo</a>
            \\      &middot;
            \\      <a rel="me" href="https://spezi.social/@firechicken">Mastodon</a>
            \\      &middot;
            \\      Commit <a href="https://github.com/bahlo/firechicken.club/commit/{s}">{s}</a>
            \\      &middot;
            \\      <a href="/colophon">Colophon</a>
            \\    </footer>
            \\  </body>
            \\</html>
        , .{ self.git_sha, self.git_sha[0..7] });
    }

    pub fn write_index(self: Templates, writer: anytype) !void {
        try self.write_header(writer, "Fire Chicken Webring", "An invite-only webring for personal websites.", "https://firechicken.club");
        const first_slug = members[0].slug;
        try writer.print(
            \\       <header class="hero">
            \\         <h1 class="hero__heading">
            \\            Fire
            \\            <br />
            \\            Chicken
            \\            <br />
            \\            Webring
            \\        </h1>
            \\        <div class="hero__fire_chicken">
            \\          <img src="/fire-chicken.svg" alt="A chicken with sunglasses and a tail of fire" />
            \\        </div>
            \\      </header>
            \\      <div class="main stack">
            \\        <p class="description">An invite-only webring for personal websites.</p>
            \\        <div>
            \\          This is what it looks like:
            \\          <a class="no-underline" href="/{s}/prev">←</a>&nbsp;<a href="https://firechicken.club">Fire&nbsp;Chicken&nbsp;Webring</a>&nbsp;<a class="no-underline" href="/{s}/next">→</a>
            \\          or
            \\          <a class="no-underline" href="/{s}/prev">←</a>&nbsp;<a class="no-underline" href="https://firechicken.club">🔥&#8288;🐓</a >&nbsp;<a class="no-underline" href="/{s}/next">→</a>
            \\        </div>
            \\        <table class="members">
            \\          <thead>
            \\            <th>Slug</th>
            \\            <th>Name</th>
            \\            <th>Url</th>
            \\          </thead>
            \\          <tbody>
        , .{ first_slug, first_slug, first_slug, first_slug });
        for (members) |member| {
            const class = if (member.invalid) "line-through" else "";
            try writer.print(
                \\            <tr class="{s}">
                \\              <td>{s}</td>
                \\              <td>{s}</td>
                \\              <td><a href="{s}">{s}</a></td>
                \\            </tr>
            , .{ class, member.slug, member.name, member.url, member.host });
        }
        try writer.print(
            \\          </tbody>
            \\        </table>
            \\        <h2>FAQ</h2>
            \\        <section class="stack-small">
            \\            <details>
            \\                <summary>What is a webring?</summary>
            \\                <p>
            \\                    A webring is a collection of website, usually grouped by a
            \\                    topic, so people that want to find websites with similar content
            \\                    can find those easily. They were popular in the 90s due to bad
            \\                    search engines. Now they’re <em>niche</em>.
            \\                </p>
            \\            </details>
            \\            <details>
            \\                <summary>Can I subscribe to all websites at once?</summary>
            \\                <p>
            \\                    Yes, there's an <a href="/opml.xml">OPML file</a> you can import
            \\                    into your RSS reader to subscribe to all sites at once.
            \\                </p>
            \\            </details>
            \\            <details>
            \\                <summary>How do I join?</summary>
            \\                <div class="stack-small">
            \\                    <p>
            \\                        If a friend of yours is in the webring, ask them to send me
            \\                        an email with your email address and your website. Once
            \\                        you're in, add the following snippet somewhere, replacing
            \\                        <code>:slug</code> with your slug:
            \\                    </p>
            \\                    <pre><code>&lt;a href=&quot;https://firechicken.club/:slug/prev&quot;&gt;←&lt;/a&gt;
            \\&lt;a href=&quot;https://firechicken.club&quot;&gt;Fire Chicken Webring&lt;/a&gt;
            \\&lt;a href=&quot;https://firechicken.club/:slug/next&quot;&gt;→&lt;/a&gt;</code></pre>
            \\                </div>
            \\            </details>
            \\            <details>
            \\                <summary>Why are some members crossed out?</summary>
            \\                <p>
            \\                    When the links are missing or the site is down for a longer
            \\                    period of time, the member is marked as invalid and skipped in
            \\                    the ring. If you're a member and you think you're marked as
            \\                    invalid by mistake, please contact me.
            \\                </p>
            \\            </details>
            \\        </section>
            \\      </div>
        , .{});
        try self.write_footer(writer);
    }

    pub fn write_not_found(self: Templates, writer: anytype) !void {
        try self.write_header(writer, "Not Found — Fire Chicken Webring", "This page could not be found.", "https://firechicken.club/404");
        try writer.print(
            \\      <a href="/">← Index</a>
            \\      <h2>404 — Not Found</h2>
            \\      <p>The page you were looking for was not found.</p>
        , .{});
        try self.write_footer(writer);
    }

    pub fn write_colophon(self: Templates, writer: anytype) !void {
        try self.write_header(writer, "Colophon — Fire Chicken Webring", "The colophon for the Fire Chicken Webring.", "https://firechicken.club/colophon");
        try writer.print(
            \\      <a href="/">← Index</a>
            \\      <h2>Colophon</h2>
            \\      <p>This website was first published on November 13th, 2023 near <a href="https://frankfurt.de">Frankfurt, Germany</a>.
            \\        It's developed on a 2021 MacBook Pro using a custom Zig application and hosted on <a href="https://netlify.com">Netlify</a>.
            \\        The code is hosted on <a href="https://github.com/bahlo/firechicken.club">GitHub</a>.</p>
            \\      <p>Testing was conducted in the latest versions of Edge, Chrome, Firefox, and Safari. Any issue you encounter on this website can be submitted as
            \\        <a href="https://github.com/bahlo/firechicken.club/issues/new">GitHub issues</a>.</p>
            \\      <p>The logo was made for this website by <a href="https://www.instagram.com/ekkapranova/">Ekaterina Kapranova</a>.</p>
            \\      <p>The font in the header is <a class="obviously-condensed" href="https://ohnotype.co/fonts/obviously">Obviously Condensed</a> by the Ohno Type Company.</p>
        , .{});
        try self.write_footer(writer);
    }

    pub fn write_opml(self: Templates, writer: anytype) !void {
        try writer.print(
            \\<opml version="1.0">
            \\    <head>
            \\        <title>{s}</title>
            \\        <date_created>{s}</date_created>
            \\    </head>
            \\    <body>
        , .{ "RSS Feeds for all Fire Chicken Webring members", self.date_created });
        for (members) |member| {
            for (member.rss_feeds) |rss_feed| {
                const title = if (rss_feed.title != null) rss_feed.title.? else member.name;
                const html_url = if (rss_feed.html_url != null) rss_feed.html_url.? else member.url;
                try writer.print(
                    \\        <outline text="{s}" title="{s}" type="rss" xmlUrl="{s}" htmlUrl="{s}" />
                , .{ title, title, rss_feed.xml_url, html_url });
            }
        }
        // <outline text="{{title}}" title="{{title}}" type="rss" xmlUrl="{{xml_url}}" htmlUrl="{{html_url}}" />
        try writer.print(
            \\    </body>
            \\</opml>
        , .{});
    }
};
