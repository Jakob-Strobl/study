const std = @import("std");

const stdout = std.io.getStdOut().writer();
pub fn main() !void {
    const immutableNegative: i32 = -1;
    // Cast fails as expected - since u32 doesn't represent negative values
    // const inferredNegative = @as(u32, immutableNegative);
    const inferredNegative = @as(i32, immutableNegative);

    // Why did is JS devs get called out in these docs LMAO
    // @see https://zig.guide/language-basics/assignment
    var undefinedUnc: i32 = undefined;
    // Can't use it until assigned
    // undefinedUnc + 2;

    // You can print it, the values look like addresses?
    // ex: 1807740656, 1871032048, 1860808432, 1869557488
    try stdout.print("{d}\n", .{undefinedUnc});
    // It's false - makes sense. The docs say it's
    try stdout.print("{any}\n", .{undefinedUnc == undefined});
    try stdout.print("{any}\n", .{undefinedUnc == 2});

    undefinedUnc = 2;
    try stdout.print("{d}\n", .{undefinedUnc});
    // It's false - makes sense. The docs say it's
    try stdout.print("{any}\n", .{undefinedUnc == undefined});
    try stdout.print("{any}\n", .{undefinedUnc == 2});

    // {d} = decimal
    // {s} = string
    // {any} = any - default formatting
    try stdout.print("immutable boiiiii - {d} {d}\n", .{ immutableNegative, inferredNegative });
    try stdout.print("Hello, Zig!\n", .{});

    try arrays();
}

pub fn arrays() !void {
    // inferred length for array literals
    const a = [_]u8{ '2', '1', '3' };
    const a_length = a.len;

    try stdout.print("[]: {any} | {d}\n", .{ a, a_length });
}
