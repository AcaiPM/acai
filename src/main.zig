const std = @import("std");
const log = @import("utils/logging.zig");
const os = std.os;

pub fn main() !void {
    log.write(.info, "Welcome to Acai!\n", .{});
    return;
}
