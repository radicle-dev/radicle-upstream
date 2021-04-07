import { Error, fromUnknown, Code } from "./error";

describe("fromUnknown", () => {
  test("returns Error as-is", () => {
    const error = new Error({
      code: Code.UnknownException,
      message: "foo",
    });
    const wrappedError = fromUnknown(error);
    expect(wrappedError).toBe(error);
  });

  test("wraps built-in error with custom props", () => {
    const error = new globalThis.Error("MESSAGE");
    const details = {
      name: "MyError",
      foo: "bar",
      qux: true,
    };
    Object.assign(error, details);
    const wrappedError = fromUnknown(error, Code.BackendTerminated);
    expect(wrappedError.code).toBe(Code.BackendTerminated);
    expect(wrappedError.message).toBe(error.message);
    expect(wrappedError.stack).toBe(error.stack);
    expect(wrappedError.details).toEqual(details);
  });

  test("wraps some arbitrary data", () => {
    const reason = { foo: "bar", qux: true };
    const wrappedError = fromUnknown(reason, Code.BackendTerminated, "MESSAGE");
    expect(wrappedError.code).toBe(Code.BackendTerminated);
    expect(wrappedError.message).toBe("MESSAGE");
    expect(wrappedError.stack).toBeDefined();
    expect(wrappedError.details).toEqual(reason);
  });
});
