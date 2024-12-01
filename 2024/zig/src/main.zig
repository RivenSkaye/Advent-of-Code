const std = @import("std");
const stdout = std.io.getStdOut().writer();
const d01 = @import("day01.zig");
const max_days: u8 = 1;

pub fn main() !void {
    for (0..max_days) |day| {
        try stdout.print("{}", .{day + 1});
    }
}
