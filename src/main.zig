const std = @import("std");
const fs = std.fs;
const io = std.io;
const Child = std.process.Child;
const ArrayList = std.ArrayList;
const Blake3 = std.crypto.hash.Blake3;
const Allocator = std.mem.Allocator;

const mustache = @import("mustache");

const members = @import("members.zig");
const Member = members.Member;
const RssFeed = members.RssFeed;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        if (gpa.detectLeaks()) @panic("Detected memory leak");
        _ = gpa.deinit();
    }
    const allocator = gpa.allocator();

    const Meta = struct {
        css_hash: []const u8,
        git_sha: []const u8,
        git_sha_short: []const u8,
    };

    const Context = struct {
        title: []const u8,
        description: []const u8,
        url: []const u8,
        meta: Meta,
        members: []const Member,
    };

    const cwd = try fs.cwd().realpathAlloc(allocator, ".");
    defer allocator.free(cwd);

    // hash css file for cache busting
    const css_path = try fs.path.join(allocator, &.{ cwd, "static", "style.css" });
    defer allocator.free(css_path);
    const css = try fs.cwd().readFileAlloc(allocator, css_path, std.math.maxInt(usize));
    defer allocator.free(css);
    var css_blake3_hash: [32]u8 = undefined;
    Blake3.hash(css, css_blake3_hash[0..], .{});
    const css_hash = std.fmt.bytesToHex(&css_blake3_hash, .lower)[0..16];

    // get git sha
    const git_sha_result = try Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "git", "rev-parse", "HEAD" },
    });
    defer allocator.free(git_sha_result.stdout);
    defer allocator.free(git_sha_result.stderr);
    if (git_sha_result.term.Exited != 0) {
        @panic("git rev-parse HEAD failed");
    }
    const git_sha = git_sha_result.stdout;

    const meta = Meta{
        .css_hash = css_hash[0..],
        .git_sha = git_sha,
        .git_sha_short = git_sha[0..7],
    };

    // MARK: Parse templates

    const header_path = try fs.path.join(allocator, &.{ cwd, "templates", "header.html" });
    defer allocator.free(header_path);
    const header_template_result = try mustache.parseFile(allocator, header_path, .{}, .{});
    const header_template = switch (header_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer header_template.deinit(allocator);

    const footer_path = try fs.path.join(allocator, &.{ cwd, "templates", "footer.html" });
    defer allocator.free(footer_path);
    const footer_template_result = try mustache.parseFile(allocator, footer_path, .{}, .{});
    const footer_template = switch (footer_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer footer_template.deinit(allocator);

    const partials = .{
        .{ "header", header_template },
        .{ "footer", footer_template },
    };

    const index_path = try fs.path.join(allocator, &.{ cwd, "templates", "index.html" });
    defer allocator.free(index_path);
    const index_template_result = try mustache.parseFile(allocator, index_path, .{}, .{});
    const index_template = switch (index_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer index_template.deinit(allocator);

    const not_found_path = try fs.path.join(allocator, &.{ cwd, "templates", "404.html" });
    defer allocator.free(not_found_path);
    const not_found_template_result = try mustache.parseFile(allocator, not_found_path, .{}, .{});
    const not_found_template = switch (not_found_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer not_found_template.deinit(allocator);

    const colophon_path = try fs.path.join(allocator, &.{ cwd, "templates", "colophon.html" });
    defer allocator.free(colophon_path);
    const colophon_template_result = try mustache.parseFile(allocator, colophon_path, .{}, .{});
    const colophon_template = switch (colophon_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer colophon_template.deinit(allocator);

    const opml_path = try fs.path.join(allocator, &.{ cwd, "templates", "opml.xml" });
    defer allocator.free(opml_path);
    const opml_template_result = try mustache.parseFile(allocator, opml_path, .{}, .{});
    const opml_template = switch (opml_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer opml_template.deinit(allocator);

    // MARK: Render to files

    fs.cwd().deleteTree("dist") catch unreachable;
    try fs.cwd().makeDir("dist");

    var dist_dir = try fs.cwd().openDir("dist", .{});
    defer dist_dir.close();

    var index_file = try dist_dir.createFile("index.html", .{});
    defer index_file.close();
    try mustache.renderPartials(index_template, partials, Context{
        .title = "Fire Chicken Webring",
        .description = "An invite-only webring for personal websites.",
        .url = "https://firechicken.club",
        .members = &members.members,
        .meta = meta,
    }, index_file.writer());

    var not_found_file = try dist_dir.createFile("404.html", .{});
    defer not_found_file.close();
    try mustache.renderPartials(not_found_template, partials, Context{
        .title = "Not Found — Fire Chicken Webring",
        .description = "This page could not be found.",
        .url = "https://firechicken.club/404",
        .members = &members.members,
        .meta = meta,
    }, not_found_file.writer());

    // collect all rss feeds into one arraylist, the templating engine panics
    // if you do nested loops
    // plus we need to fill in title/html_url if null
    var rss_feed_list = ArrayList(RssFeed).init(allocator);
    defer rss_feed_list.deinit();
    for (members.members) |member| {
        for (member.rss_feeds) |rss_feed| {
            var rss_feed_copy = rss_feed;
            if (rss_feed_copy.title == null) {
                rss_feed_copy.title = member.name;
            }
            if (rss_feed_copy.html_url == null) {
                rss_feed_copy.html_url = member.url;
            }
            try rss_feed_list.append(rss_feed_copy);
        }
    }
    const rss_feeds = try rss_feed_list.toOwnedSlice();
    defer allocator.free(rss_feeds);
    var opml_file = try dist_dir.createFile("opml.xml", .{});
    defer opml_file.close();
    try mustache.render(opml_template, .{
        .title = "RSS Feeds for all Fire Chicken Webring members",
        .date_created = "lol fix me",
        .rss_feeds = rss_feeds,
    }, opml_file.writer());

    try dist_dir.makeDir("colophon");
    var colophon_dir = try dist_dir.openDir("colophon", .{});
    defer colophon_dir.close();
    var colophon_file = try colophon_dir.createFile("index.html", .{});
    defer colophon_file.close();
    try mustache.renderPartials(colophon_template, partials, Context{
        .title = "Colophon — Fire Chicken Webring",
        .description = "The colophon for the Fire Chicken Webring.",
        .url = "https://firechicken.club/colophon",
        .members = &members.members,
        .meta = meta,
    }, colophon_file.writer());

    // MARK: Render _redirects

    var redirects_file = try dist_dir.createFile("_redirects", .{});
    defer redirects_file.close();
    try renderRedirects(redirects_file.writer(), &members.members);

    // MARK: Copy assets

    var static_dir = try fs.cwd().openDir("static", .{});
    defer static_dir.close();
    try copyDirRecursive(static_dir, dist_dir);
}

fn copyDirRecursive(src_dir: fs.Dir, dest_dir: fs.Dir) !void {
    var iter = src_dir.iterate();

    // no unbound while loops, 256 is enough for anyone
    var i: u8 = 0;
    while (i < std.math.maxInt(u8)) : (i += 1) {
        const entry = try iter.next();
        if (entry == null) {
            break;
        }

        switch (entry.?.kind) {
            .file => try src_dir.copyFile(entry.?.name, dest_dir, entry.?.name, .{}),
            .directory => {
                try dest_dir.makeDir(entry.?.name);

                const open_dir_options = fs.Dir.OpenDirOptions{
                    .access_sub_paths = true,
                    .iterate = true,
                    .no_follow = true,
                };

                var src_entry_dir = try src_dir.openDir(entry.?.name, open_dir_options);
                defer src_entry_dir.close();
                var dest_entry_dir = try dest_dir.openDir(entry.?.name, open_dir_options);
                defer dest_entry_dir.close();
                try copyDirRecursive(src_entry_dir, dest_entry_dir);
            },
            else => @panic("non-file/directory entry in static directory"),
        }
    }
}

fn renderRedirects(writer: anytype, member_list: []const Member) !void {
    var prev_member = member_list[member_list.len - 1];
    var last_invalid_member: ?Member = null;
    for (member_list) |member| {
        // always redirect to prev member, even from invalid slugs
        try writer.print("/{s}/prev {s} 302\n", .{member.slug, prev_member.url});

        if (last_invalid_member != null) {
            try writer.print("/{s}/next {s} 302\n", .{last_invalid_member.?.slug, member.url});
            last_invalid_member = null;
        }

        if (member.invalid) {
            if (last_invalid_member != null) {
                @panic("two consecutive invalid members are not supported, have fun fixing this");
            }

            last_invalid_member = member;
            continue;
        }

        try writer.print("/{s}/next {s} 302\n", .{prev_member.slug, member.url});

        prev_member = member;
    }
}

test "redirects render correctly"  {
    var buf = ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();

    const member_list = [_]Member{
        Member{.slug = "foo", .name = "foo", .url = "https://foo.test", .host = "foo.test", .invalid = false, .joined = "2024-04-30", .rss_feeds = &[_]RssFeed{}},
        Member{.slug = "bar", .name = "bar", .url = "https://bar.test", .host = "bar.test", .invalid = false, .joined = "2024-04-30", .rss_feeds = &[_]RssFeed{}},
        Member{.slug = "baz", .name = "baz", .url = "https://baz.test", .host = "baz.test", .invalid = false, .joined = "2024-04-30", .rss_feeds = &[_]RssFeed{}},
    };

    try renderRedirects(buf.writer(), &member_list);
    const actual = try buf.toOwnedSlice();
    defer std.testing.allocator.free(actual);

    try std.testing.expectEqualStrings(
    \\/foo/prev https://baz.test 302
    \\/baz/next https://foo.test 302
    \\/bar/prev https://foo.test 302
    \\/foo/next https://bar.test 302
    \\/baz/prev https://bar.test 302
    \\/bar/next https://baz.test 302
    \\
    , actual);
}

test "invalid members are skipped in redirects"  {
    var buf = ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();

    const member_list = [_]Member{
        Member{.slug = "foo", .name = "foo", .url = "https://foo.test", .host = "foo.test", .invalid = false, .joined = "2024-04-30", .rss_feeds = &[_]RssFeed{}},
        Member{.slug = "bar", .name = "bar", .url = "https://bar.test", .host = "bar.test", .invalid = true, .joined = "2024-04-30", .rss_feeds = &[_]RssFeed{}},
        Member{.slug = "baz", .name = "baz", .url = "https://baz.test", .host = "baz.test", .invalid = false, .joined = "2024-04-30", .rss_feeds = &[_]RssFeed{}},
    };

    try renderRedirects(buf.writer(), &member_list);
    const actual = try buf.toOwnedSlice();
    defer std.testing.allocator.free(actual);

    try std.testing.expectEqualStrings(
    \\/foo/prev https://baz.test 302
    \\/baz/next https://foo.test 302
    \\/bar/prev https://foo.test 302
    \\/baz/prev https://foo.test 302
    \\/bar/next https://baz.test 302
    \\/foo/next https://baz.test 302
    \\
    , actual);
}
