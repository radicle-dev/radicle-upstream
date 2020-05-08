import hotkeys from "hotkeys-js";
import { get } from "svelte/store";
import { push, pop, location } from "svelte-spa-router";

import * as path from "../src/path.ts";
import { colorConfig } from "../../tokens/colors.js";

const hotModal = (key, destinationPath) =>
  hotkeys(key, () => {
    if (path.active(destinationPath, get(location))) {
      pop();
    }
    push(destinationPath);
  });

export const initializeHotkeys = () => {
  hotModal("shift+d", path.designSystemGuide());
  hotModal("shift+/", path.help());

  hotkeys("shift+c", () => {
    const currentTheme = document.documentElement.getAttribute("data-theme");

    const nextTheme = colorConfig.themes
      .map((theme) => theme.name)
      .filter((e) => e !== currentTheme)
      .pop();
    document.documentElement.setAttribute("data-theme", nextTheme);

    console.log(`Switch to ${nextTheme}`);
  });

  // TODO(sos): rethink how we do esc hotkey; any time we show a modal, it should
  // be dismissable with esc. this can be incorporated into Modal component
  hotkeys("esc", () => {
    if (
      path.active(path.help(), get(location)) ||
      path.active(path.designSystemGuide(), get(location)) ||
      path.active(path.createProject(), get(location)) ||
      path.active(path.registerProject("**"), get(location), true) ||
      path.active(path.registerUser(), get(location)) ||
      path.active(path.transactions("**"), get(location)) ||
      path.active(path.orgRegistration(), get(location)) ||
      path.active(path.addMember("**"), get(location))
    ) {
      pop();
    }
  });
};
