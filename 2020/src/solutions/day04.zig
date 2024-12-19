const std = @import("std");
const helpers = @import("../helpers.zig");

pub fn part1(allocator: std.mem.Allocator, path: []const u8) !u32 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    var map = std.AutoHashMap(u8, u32).init(allocator);
    defer map.deinit();

    var result: u32 = 0;
    var bad: bool = false;
    var content_it = std.mem.tokenizeSequence(u8, content, "\n\n");
    while (content_it.next()) |line| {
        try map.put(0, 0);
        try map.put(1, 0);
        try map.put(2, 0);
        try map.put(3, 0);
        try map.put(4, 0);
        try map.put(5, 0);
        try map.put(6, 0);
        try map.put(7, 0);

        var passport_it = std.mem.splitAny(u8, line, "\n");
        while (passport_it.next()) |passport| {
            bad = false;
            const trimmed = std.mem.trim(u8, passport, &std.ascii.whitespace);
            if (trimmed.len == 0) continue;
            var entry_it = std.mem.splitAny(u8, trimmed, " ");
            while (entry_it.next()) |entry| {
                const key = try std.fmt.allocPrint(allocator, "{c}{c}{c}", .{ entry[0], entry[1], entry[2] });
                defer allocator.free(key);

                if (std.mem.eql(u8, key, "byr")) {
                    try map.put(0, 1);
                } else if (std.mem.eql(u8, key, "iyr")) {
                    try map.put(1, 1);
                } else if (std.mem.eql(u8, key, "eyr")) {
                    try map.put(2, 1);
                } else if (std.mem.eql(u8, key, "hgt")) {
                    try map.put(3, 1);
                } else if (std.mem.eql(u8, key, "hcl")) {
                    try map.put(4, 1);
                } else if (std.mem.eql(u8, key, "ecl")) {
                    try map.put(5, 1);
                } else if (std.mem.eql(u8, key, "pid")) {
                    try map.put(6, 1);
                } else {
                    continue;
                }
            }
        }

        for (0..7) |i| {
            const value = map.get(@intCast(i)).?;
            if (value < 1) {
                bad = true;
                break;
            }
        }

        if (!bad) result += 1;
    }

    return result;
}

pub fn part2(allocator: std.mem.Allocator, path: []const u8) !u32 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);
    defer allocator.free(content);

    var map = std.AutoHashMap(u8, u32).init(allocator);
    defer map.deinit();

    var result: u32 = 0;
    var bad: bool = false;
    var content_it = std.mem.tokenizeSequence(u8, content, "\n\n");
    while (content_it.next()) |line| {
        try map.put(0, 0);
        try map.put(1, 0);
        try map.put(2, 0);
        try map.put(3, 0);
        try map.put(4, 0);
        try map.put(5, 0);
        try map.put(6, 0);
        try map.put(7, 0);

        var passport_it = std.mem.splitAny(u8, line, "\n");
        while (passport_it.next()) |passport| {
            bad = false;
            const trimmed = std.mem.trim(u8, passport, &std.ascii.whitespace);
            if (trimmed.len == 0) continue;
            var entry_it = std.mem.splitAny(u8, trimmed, " ");
            while (entry_it.next()) |entry| {
                const key = try std.fmt.allocPrint(allocator, "{c}{c}{c}", .{ entry[0], entry[1], entry[2] });
                defer allocator.free(key);

                const value = entry[4..];

                if (std.mem.eql(u8, key, "byr")) {
                    const year = try std.fmt.parseInt(u32, value, 10);
                    if (value.len < 4) continue;
                    if (year < 1920 or year > 2002) continue;

                    const prev = map.get(0);
                    try map.put(0, @intCast(prev.? + 1));
                } else if (std.mem.eql(u8, key, "iyr")) {
                    const year = try std.fmt.parseInt(u32, value, 10);
                    if (value.len < 4) continue;
                    if (year < 2010 or year > 2020) continue;

                    const prev = map.get(1);
                    try map.put(1, @intCast(prev.? + 1));
                } else if (std.mem.eql(u8, key, "eyr")) {
                    const year = try std.fmt.parseInt(u32, value, 10);
                    if (value.len < 4) continue;
                    if (year < 2020 or year > 2030) continue;

                    const prev = map.get(2);
                    try map.put(2, @intCast(prev.? + 1));
                } else if (std.mem.eql(u8, key, "hgt")) {
                    var digits = std.ArrayList(u8).init(allocator);
                    defer digits.deinit();
                    var chars = std.ArrayList(u8).init(allocator);
                    defer chars.deinit();

                    for (value) |c| {
                        if (std.ascii.isDigit(c)) {
                            try digits.append(c);
                        } else if (std.ascii.isAlphabetic(c)) {
                            try chars.append(c);
                        }
                    }

                    const height = try std.fmt.parseInt(u32, digits.items, 10);
                    if (std.mem.eql(u8, chars.items, "in")) {
                        if (height < 59 or height > 76) continue;
                    } else if (std.mem.eql(u8, chars.items, "cm")) {
                        if (height < 150 or height > 193) continue;
                    } else {
                        continue;
                    }

                    const prev = map.get(3);
                    try map.put(3, @intCast(prev.? + 1));
                } else if (std.mem.eql(u8, key, "hcl")) {
                    var valid = true;
                    if (value.len != 7) continue;
                    const hashtag = value[0];
                    if (!std.mem.eql(u8, &[_]u8{hashtag}, "#")) continue;

                    for (value[1..7]) |c| {
                        if ((c < '0' or c > '9') and (c < 'a' or c > 'f')) {
                            valid = false;
                            break;
                        }
                    }

                    if (!valid) continue;
                    const prev = map.get(4);
                    try map.put(4, @intCast(prev.? + 1));
                } else if (std.mem.eql(u8, key, "ecl")) {
                    if (std.mem.eql(u8, value, "amb") or std.mem.eql(u8, value, "blu") or std.mem.eql(u8, value, "brn") or std.mem.eql(u8, value, "gry") or std.mem.eql(u8, value, "grn") or std.mem.eql(u8, value, "hzl") or std.mem.eql(u8, value, "oth")) {
                        const prev = map.get(5);
                        try map.put(5, @intCast(prev.? + 1));
                    } else {
                        continue;
                    }
                } else if (std.mem.eql(u8, key, "pid")) {
                    if (value.len != 9) continue;
                    var valid = true;
                    for (value) |c| {
                        if (!std.ascii.isDigit(c)) {
                            valid = false;
                            break;
                        }
                    }

                    const prev = map.get(6);
                    if (valid) try map.put(6, @intCast(prev.? + 1));
                } else {
                    continue;
                }
            }
        }

        for (0..7) |i| {
            const value = map.get(@intCast(i)).?;
            if (value < 1 or value > 1) {
                bad = true;
                break;
            }
        }

        if (!bad) result += 1;
    }

    return result;
}
