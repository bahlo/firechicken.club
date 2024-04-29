const std = @import("std");
const Child = std.process.Child;
const ArrayList = std.ArrayList;
const Blake3 = std.crypto.hash.Blake3;

const mustache = @import("mustache");

const members = @import("members.zig");
const Member = members.Member;

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

    const cwd = try std.fs.cwd().realpathAlloc(allocator, ".");
    defer allocator.free(cwd);

    // hash css file for cache busting
    const css_path = try std.fs.path.join(allocator, &.{ cwd, "static", "style.css" });
    defer allocator.free(css_path);
    const css = try std.fs.cwd().readFileAlloc(allocator, css_path, std.math.maxInt(usize));
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

    const header_path = try std.fs.path.join(allocator, &.{ cwd, "templates", "header.html" });
    defer allocator.free(header_path);
    const header_template_result = try mustache.parseFile(allocator, header_path, .{}, .{});
    const header_template = switch (header_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer header_template.deinit(allocator);

    const footer_path = try std.fs.path.join(allocator, &.{ cwd, "templates", "footer.html" });
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

    const index_path = try std.fs.path.join(allocator, &.{ cwd, "templates", "index.html" });
    defer allocator.free(index_path);
    const index_template_result = try mustache.parseFile(allocator, index_path, .{}, .{});
    const index_template = switch (index_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer index_template.deinit(allocator);

    const not_found_path = try std.fs.path.join(allocator, &.{ cwd, "templates", "404.html" });
    defer allocator.free(not_found_path);
    const not_found_template_result = try mustache.parseFile(allocator, not_found_path, .{}, .{});
    const not_found_template = switch (not_found_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer not_found_template.deinit(allocator);

    const colophon_path = try std.fs.path.join(allocator, &.{ cwd, "templates", "colophon.html" });
    defer allocator.free(colophon_path);
    const colophon_template_result = try mustache.parseFile(allocator, colophon_path, .{}, .{});
    const colophon_template = switch (colophon_template_result) {
        .parse_error => |parse_error| return parse_error.parse_error,
        .success => |template| blk: {
            break :blk template;
        },
    };
    defer colophon_template.deinit(allocator);

    // MARK: Render to files

    std.fs.cwd().deleteTree("dist") catch unreachable;
    try std.fs.cwd().makeDir("dist");

    var dist_dir = try std.fs.cwd().openDir("dist", .{});
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

    // MARK: Copy assets
}
