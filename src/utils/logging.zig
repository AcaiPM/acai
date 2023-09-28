const std = @import("std");

var log_mutex = std.Thread.Mutex{};
pub const log_type = enum {
    info,
    warning,
    err,
    debug,
    none,

    pub fn str(level: log_type) []const u8 {
        return switch (level) {
            .info => "Info",
            .warning => "Warning",
            .err => "Error",
            .debug => "Debug",
            .none => "",
        };
    }
};

pub fn write(level: log_type, comptime fmt: []const u8, args: anytype) void {
    log_mutex.lock();
    defer log_mutex.unlock();

    switch (level) {
        .info, .debug, .none => {
            const stdout = std.io.getStdOut().writer();
            if (level != .none) {
                nosuspend stdout.print("[{s}]: ", .{log_type.str(level)}) catch return;
            }
            nosuspend stdout.print(fmt, args) catch return;
        },
        .warning, .err => {
            const stderr = std.io.getStdErr().writer();
            nosuspend stderr.print("[{s}]: ", .{log_type.str(level)}) catch return;
            nosuspend stderr.print(fmt, args) catch return;
        },
    }
}