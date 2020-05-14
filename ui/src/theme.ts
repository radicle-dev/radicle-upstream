import * as colors from "../../tokens/colors.js";

import * as session from "./session";

export enum Theme {
  Dark = "dark",
  Light = "light",
}

export const next = (): Theme => {
  const currentTheme = document.documentElement.getAttribute("data-theme");
  const next = colors.colorConfig.themes
    .map((theme) => theme.name)
    .filter((e) => e !== currentTheme)
    .pop();

  return next ? next as Theme : Theme.Light;
};

session.settings.subscribe((settings) => {
  if (settings) {
    const theme = settings.appearance.theme;
    document.documentElement.setAttribute("data-theme", theme);
    console.log(`Switch to ${theme}`);
  }
});
