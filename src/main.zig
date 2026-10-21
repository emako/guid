const std = @import("std");
const Io = std.Io;

fn hexByte(byte: u8, buf: *[2]u8) void {
    const chars = "0123456789abcdef";
    buf[0] = chars[@as(usize, @intCast(byte >> 4))];
    buf[1] = chars[@as(usize, @intCast(byte & 0x0f))];
}

fn hexSlice(bytes: []const u8, buf: []u8) void {
    for (bytes, 0..) |b, i| {
        const h = @as(usize, @intCast(i * 2));
        const chars = "0123456789abcdef";
        buf[h] = chars[@as(usize, @intCast(b >> 4))];
        buf[h + 1] = chars[@as(usize, @intCast(b & 0x0f))];
    }
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;

    var buf: [16]u8 = undefined;
    io.random(&buf);

    buf[6] = (buf[6] & 0x0f) | 0x40;
    buf[8] = (buf[8] & 0x3f) | 0x80;

    var hex: [36]u8 = undefined;
    hexSlice(buf[0..4], hex[0..8]);
    hex[8] = '-';
    hexSlice(buf[4..6], hex[9..13]);
    hex[13] = '-';
    hexSlice(buf[6..8], hex[14..18]);
    hex[18] = '-';
    hexSlice(buf[8..10], hex[19..23]);
    hex[23] = '-';
    hexSlice(buf[10..], hex[24..]);

    var stdout_buf: [512]u8 = undefined;
    var stdout_writer = Io.File.stdout().writerStreaming(io, &stdout_buf);
    const stdout = &stdout_writer.interface;

    try stdout.print("{{{s}}}\n", .{hex});
    try stdout.flush();
}
