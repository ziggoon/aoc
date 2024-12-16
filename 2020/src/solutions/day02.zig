const std = @import("std");
const helpers = @import("../helpers.zig");

pub fn part1(allocator: std.mem.Allocator, path: []const u8) !u32 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    var line_it = std.mem.tokenizeSequence(u8, content, "\n");
    var total: u32 = 0;
    while (line_it.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;

        var map = std.AutoHashMap(u8, u32).init(allocator);
        defer map.deinit();

        var entry = std.mem.tokenizeSequence(u8, trimmed, " ");
        var policy_it = std.mem.splitAny(u8, entry.next().?, "-");

        const min = try std.fmt.parseInt(u32, policy_it.next().?, 10);
        const max = try std.fmt.parseInt(u32, policy_it.next().?, 10);
        const char = entry.next().?[0];
        const password = entry.next().?;

        for (password) |character| {
            const result = try map.getOrPut(character);
            if (!result.found_existing) {
                result.value_ptr.* = 1;
            } else {
                result.value_ptr.* += 1;
            }
        }

        if (map.get(char)) |count| {
            if (count >= min and count <= max) {
                total += 1;
            }
        }
    }

    return total;
}

pub fn part2(allocator: std.mem.Allocator, path: []const u8) !u32 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    var line_it = std.mem.tokenizeSequence(u8, content, "\n");
    var total: u32 = 0;
    while (line_it.next()) |line| {
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;

        var map = std.AutoHashMap(u8, u32).init(allocator);
        defer map.deinit();

        var count: u32 = 0;
        var entry = std.mem.tokenizeSequence(u8, trimmed, " ");
        var policy_it = std.mem.splitAny(u8, entry.next().?, "-");

        const index1 = try std.fmt.parseInt(u32, policy_it.next().?, 10);
        const index2 = try std.fmt.parseInt(u32, policy_it.next().?, 10);
        const char = entry.next().?[0];
        const password = entry.next().?;

        for (password, 1..) |character, index| {
            if ((index == index1 or index == index2) and char == character) {
                count += 1;
            }
        }

        if (count == 1) total += 1;
    }

    return total;
}
