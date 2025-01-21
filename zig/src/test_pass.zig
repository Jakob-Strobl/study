const std = @import("std");
const expect = std.testing.expect;

test "always works" {
    // Without try. the test fails because expect() returns a union
    //   Union is (Error | Not Error) I'm guessing?
    // Smart way to force accurate tests and improve error handling in code
    try expect(true);
}

test "always failes" {
    // Without try. the test fails because expect() returns a union
    //   Union is (Error | Not Error) I'm guessing?
    // Smart way to force accurate tests and improve error handling in code
    try expect(false);
}
