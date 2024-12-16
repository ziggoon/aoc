const std = @import("std");

pub fn readInputFile(allocator: std.mem.Allocator, path: []const u8) ![]u8 {
    const content = try std.fs.cwd().readFileAlloc(allocator, path, 1024 * 1024);

    return content;
}
