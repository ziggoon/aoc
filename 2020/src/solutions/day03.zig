const std = @import("std");
const helpers = @import("../helpers.zig");

const Path = struct {
    right: u32,
    down: u32,
};

pub fn part1(allocator: std.mem.Allocator, path: []const u8) !u32 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    var content_it = std.mem.tokenizeSequence(u8, content, "\n");
    var x: u32 = 0;
    var trees: u32 = 0;
    while (content_it.next()) |line| {
        if (x == 0) {
            x += 3;
            continue;
        }
        const trimmed = std.mem.trim(u8, line, &std.ascii.whitespace);
        if (trimmed.len == 0) continue;

        if (x >= trimmed.len) x = @intCast((x - trimmed.len));
        if (trimmed[x] == '#') trees += 1;

        x += 3;
    }

    return trees;
}

pub fn part2(allocator: std.mem.Allocator, path: []const u8) !u64 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    const slopes = [5]Path{
        .{ .right = 1, .down = 1 },
        .{ .right = 3, .down = 1 },
        .{ .right = 5, .down = 1 },
        .{ .right = 7, .down = 1 },
        .{ .right = 1, .down = 2 },
    };

    var lines = std.mem.tokenizeScalar(u8, content, '\n');
    const width = lines.next().?.len;

    var result: u64 = 1;
    for (slopes) |slope| {
        lines = std.mem.tokenizeScalar(u8, content, '\n'); // Reset iterator
        const trees = try countTrees(&lines, width, slope);
        result *= trees;
    }

    return result;
}

fn countTrees(lines: *std.mem.TokenIterator(u8, .scalar), width: usize, path: Path) !u64 {
    var trees: u64 = 0;
    var x: usize = 0;
    _ = lines.next(); // Skip first line

    var y: usize = 1;
    while (lines.next()) |line| {
        if (y % path.down != 0) {
            y += 1;
            continue;
        }

        x = (x + path.right) % width;
        if (line[x] == '#') {
            trees += 1;
        }
        y += 1;
    }

    return trees;
}
