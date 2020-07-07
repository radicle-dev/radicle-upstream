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
});
