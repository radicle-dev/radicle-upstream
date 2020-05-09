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
};
