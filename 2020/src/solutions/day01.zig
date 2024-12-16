const std = @import("std");
const helpers = @import("../helpers.zig");

pub fn part1(allocator: std.mem.Allocator, path: []const u8) !u32 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    var map = std.AutoHashMap(u32, u32).init(allocator);
    defer map.deinit();

    var line_it = std.mem.tokenizeSequence(u8, content, "\n");

    while (line_it.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;

        const key = map.count();
        const value = try std.fmt.parseInt(u32, trimmed, 10);
        try map.put(@intCast(key), value);
    }

    var result: u32 = 0;
    const map_len = map.count();
    for (0..map_len) |i| {
        const item1 = map.get(@intCast(i));
        for (0..map_len - 1) |j| {
            const item2 = map.get(@intCast(j + 1));
            if (item1.? + item2.? == 2020) {
                result += (item1.? * item2.?);
                return result;
            }
        }
    }
    return error.AnswerNotFound;
}

pub fn part2(allocator: std.mem.Allocator, path: []const u8) !u32 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    var map = std.AutoHashMap(u32, u32).init(allocator);
    defer map.deinit();

    var line_it = std.mem.tokenizeSequence(u8, content, "\n");

    while (line_it.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;

        const key = map.count();
        const value = try std.fmt.parseInt(u32, trimmed, 10);
        try map.put(@intCast(key), value);
    }

    const map_len = map.count();
    for (0..map_len) |i| {
        const item1 = map.get(@intCast(i));
        for (0..map_len) |j| {
            if (i == j) continue;
            const item2 = map.get(@intCast(j));
            for (0..map_len) |k| {
                if (i == k or j == k) continue;
                const item3 = map.get(@intCast(k));

                if (item1.? + item2.? + item3.? == 2020) {
                    return item1.? * item2.? * item3.?;
                }
            }
        }
    }
    return error.AnswerNotFound;
}
