const std = @import("std");
const Allocator = std.mem.Allocator;
const json = std.json;
const fs = std.fs;

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

pub fn parse(allocator: Allocator) !json.Parsed([]Member) {
    const file = try fs.cwd().openFile("members.json", .{});
    defer file.close();

    var contents: [1024 * 1024]u8 = undefined; // 1mb should be enough?
    const len = try file.readAll(&contents);

    const parsed_members = try json.parseFromSlice([]Member, allocator, contents[0..len], .{ .allocate = .alloc_always });
    return parsed_members;
}
