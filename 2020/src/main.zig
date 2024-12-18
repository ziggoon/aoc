const std = @import("std");
const print = std.debug.print;
const testing = std.testing;

const day01 = @import("solutions/day01.zig");
const day02 = @import("solutions/day02.zig");
const day03 = @import("solutions/day03.zig");
const day04 = @import("solutions/day04.zig");

const day1_path = "input/day01.txt";
const day2_path = "input/day02.txt";
const day3_path = "input/day03.txt";
const day4_path = "input/day04.txt";

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var result: u32 = 0;
    result = try day01.part1(allocator, day1_path);
    print("[+] day 1 part 1: {d}\n", .{result});
    result = try day01.part2(allocator, day1_path);
    print("[+] day 1 part 2: {d}\n\n", .{result});

    result = try day02.part1(allocator, day2_path);
    print("[+] day 2 part 1: {d}\n", .{result});
    result = try day02.part2(allocator, day2_path);
    print("[+] day 2 part 2: {d}\n\n", .{result});

    result = try day03.part1(allocator, day3_path);
    print("[+] day 3 part 1: {d}\n", .{result});
    const result2 = try day03.part2(allocator, day3_path);
    print("[+] day 3 part 2: {d}\n\n", .{result2});

    result = try day04.part1(allocator, day4_path);
    print("[+] day 4 part 1: {d}\n", .{result});
    result = try day04.part2(allocator, day4_path);
    print("[+] day 4 part 2: {d}\n", .{result});
}

const test_path_day1 = "input/day01-test.txt";
const test_path_day2 = "input/day02-test.txt";
const test_path_day3 = "input/day03-test.txt";
const test_path_day4 = "input/day04-test.txt";
test "day 1 part 1" {
    const allocator = testing.allocator;
    const part1 = try day01.part1(allocator, test_path_day1);

    try testing.expectEqual(@as(u32, 514579), part1);
    print("[+] day 1 part 1 test ✅: {d}\n", .{part1});
}

test "day 1 part 2" {
    const allocator = testing.allocator;
    const part2 = try day01.part2(allocator, test_path_day1);

    try testing.expectEqual(@as(u32, 241861950), part2);
    print("[+] day 1 part 2 test ✅: {d}\n", .{part2});
}

test "day 2 part 1" {
    const allocator = testing.allocator;
    const part1 = try day02.part1(allocator, test_path_day2);

    try testing.expectEqual(@as(u32, 2), part1);
    print("[+] day 2 part 1 test ✅: {d}\n", .{part1});
}

test "day 2 part 2" {
    const allocator = testing.allocator;
    const part2 = try day02.part2(allocator, test_path_day2);

    try testing.expectEqual(@as(u32, 1), part2);
    print("[+] day 2 part 2 test ✅: {d}\n", .{part2});
}

test "day 3 part 1" {
    const allocator = testing.allocator;
    const part1 = try day03.part1(allocator, test_path_day3);

    try testing.expectEqual(@as(u32, 7), part1);
    print("[+] day 3 part 1 test ✅: {d}\n", .{part1});
}

test "day 3 part 2" {
    const allocator = testing.allocator;
    const part2 = try day03.part2(allocator, test_path_day3);

    try testing.expectEqual(@as(u64, 336), part2);
    print("[+] day 3 part 2 test ✅: {d}\n", .{part2});
}

test "day 4 part 1" {
    const allocator = testing.allocator;
    const part1 = try day04.part1(allocator, test_path_day4);

    try testing.expectEqual(@as(u32, 2), part1);
    print("[+] day 4 part 1 test ✅: {d}\n", .{part1});
}

test "day 4 part 2" {
    const allocator = testing.allocator;
    const part2 = try day04.part2(allocator, test_path_day4);

    try testing.expectEqual(@as(u32, 2), part2);
    print("[+] day 4 part 2 test ✅: {d}\n", .{part2});
}
