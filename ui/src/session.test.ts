import * as session from "./session";

describe("settings", () => {
  it("parses seeds correctly", () => {
    const desiredResult = ["seed.radicle.xyz", "192.134.54.13", "192.168.1.0"];

    const newlineSeparated =
      "seed.radicle.xyz\n192.134.54.13   \n\n192.168.1.0\n";
    const commaSeparated = "seed.radicle.xyz, 192.134.54.13,192.168.1.0,,,,,,";
    const carriageReturnSeparated =
      "seed.radicle.xyz\r\n192.134.54.13  \r \r\n192.168.1.0\n";

    expect(session.parseSeedsInput(newlineSeparated)).toEqual(desiredResult);
    expect(session.parseSeedsInput(commaSeparated)).toEqual(desiredResult);
    expect(session.parseSeedsInput(carriageReturnSeparated)).toEqual(
      desiredResult
    );
  });
});
