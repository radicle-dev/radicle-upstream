import * as svelteStore from "svelte/store";

import * as api from "./api";
import * as session from "./session";
import * as settings from "./settings";

jest.mock("./api");

const defaultSettings = {
  appearance: { theme: "light", hints: { showRemoteHelper: true } },
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
    const currentSettings = svelteStore.get(session.settings);
    session.updateAppearance({
      ...currentSettings.appearance,
      theme: settings.Theme.Dark,
    });

    expect(api.set).toHaveBeenCalledWith("session/settings", {
      ...currentSettings,
      appearance: { ...currentSettings.appearance, theme: settings.Theme.Dark },
    });
  });
});

describe("coco settings", () => {
  it("sends a request to update CoCo settings when updateCoCo is called", () => {
    session.updateCoCo({
      seeds: [
        "hynewpywqj6x4mxgj7sojhue3erucyexiyhobxx4du9w66hxhbfqbw@seedling.radicle.xyz:12345",
      ],
    });

    expect(api.set).toHaveBeenCalledWith("session/settings", {
      ...defaultSettings,
      coco: {
        seeds: [
          "hynewpywqj6x4mxgj7sojhue3erucyexiyhobxx4du9w66hxhbfqbw@seedling.radicle.xyz:12345",
        ],
      },
    });
  });
});
