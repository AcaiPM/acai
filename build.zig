const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.Build) !void {
    const optimize = b.standardOptimizeOption(.{});
    const target = b.standardTargetOptions(.{});

    const exe = b.addExecutable(.{
        .name = "acai",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    switch (builtin.os.tag) {
        .ios, .macos, .tvos, .watchos => {
            exe.linkFramework("CoreFoundation");
        },
        .linux => {
            exe.linkLibC();
        },
        else => {
            @compileError("Unsupported target OS");
        },
    }

    b.installArtifact(exe);
}
