import { get } from "svelte/store";

import * as api from "./api";
import * as session from "./session";
import * as settings from "./settings";

jest.mock("./api");

describe("clearing", () => {
  it("sends a request to clear the session when clear() is called", () => {
    session.clear();
    expect(api.del).toHaveBeenCalledWith("session");
  });

  it("sends a request to clear the cache when clearCache() is called", () => {
    session.clearCache();
    expect(api.del).toHaveBeenCalledWith("session/cache");
  });
});

describe("appearance settings", () => {
  it("sends a request to update appearance settings when updateAppearance() is called", () => {
    session.updateAppearance({ theme: settings.Theme.Dark });

    expect(api.set).toHaveBeenCalledWith("session/settings", {
      appearance: { theme: "dark" },
      registry: { network: "emulator" },
    });
  });
});

describe("registry settings", () => {
  it("sends a request to update registry settings when updateRegistry is called", () => {
    session.updateRegistry({ network: settings.Network.FFnet });

    expect(api.set).toHaveBeenCalledWith("session/settings", {
      appearance: { theme: "light" },
      registry: { network: "ffnet" },
    });
  });
});

describe("coco settings", () => {
  it("initially contains default seeds", () => {
    const defaultSeeds = ["seed.radicle.xyz", "194.134.54.13"];

    const store = session.seeds;
    expect(get(store).data).toEqual(defaultSeeds);
  });

  it("formats stored seeds for display", () => {
    const seedsArray = ["123.123.123", "seeds.seedy.xyz"];

    expect(session.formatSeedsForInput(seedsArray)).toEqual(
      "123.123.123\nseeds.seedy.xyz"
    );
  });

  it("parses seed textarea input when updating the store", () => {
    const desiredResult = ["seed.radicle.xyz", "192.134.54.13", "192.168.1.0"];

    const newlineSeparated =
      "seed.radicle.xyz\n192.134.54.13   \n\n192.168.1.0\n";
    const commaSeparated = "seed.radicle.xyz, 192.134.54.13,192.168.1.0,,,,,,";
    const carriageReturnSeparated =
      "seed.radicle.xyz\r\n192.134.54.13  \r \r\n192.168.1.0\n";

    session.updateSeeds(newlineSeparated);
    expect(get(session.seeds).data).toEqual(desiredResult);

    session.updateSeeds(commaSeparated);
    expect(get(session.seeds).data).toEqual(desiredResult);

    session.updateSeeds(carriageReturnSeparated);
    expect(get(session.seeds).data).toEqual(desiredResult);
  });
});
