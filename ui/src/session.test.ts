import * as api from "./api";
import * as session from "./session";
import * as settings from "./settings";

jest.mock("./api");

const defaultSettings = {
  appearance: { theme: "light" },
  coco: { seeds: ["seed.radicle.xyz"] },
};

describe("clearing", () => {
  it("sends a request to clear the session when clear() is called", () => {
    session.clear();
    expect(api.del).toHaveBeenCalledWith("session");
  });
});

describe("appearance settings", () => {
  it("sends a request to update appearance settings when updateAppearance() is called", () => {
    session.updateAppearance({ theme: settings.Theme.Dark });

    expect(api.set).toHaveBeenCalledWith("session/settings", {
      ...defaultSettings,
      appearance: { theme: settings.Theme.Dark },
    });
  });
});

describe("seed settings", () => {
  it("parses seed textarea input", () => {
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
    expect(session.parseSeedsInput("")).toEqual([]);
  });

  it("sends a request to update CoCo settings when updateCoCo is called", () => {
    session.updateCoCo({ seeds: ["new_seed.radicle.xyz"] });

    expect(api.set).toHaveBeenCalledWith("session/settings", {
      ...defaultSettings,
      coco: { seeds: ["new_seed.radicle.xyz"] },
    });
  });
});
