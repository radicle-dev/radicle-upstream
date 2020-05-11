import * as colors from "../../tokens/colors.js";

export const next = (): void => {
  const currentTheme = document.documentElement.getAttribute("data-theme");

  const nextTheme = colors.colorConfig.themes
    .map((theme) => theme.name)
    .filter((e) => e !== currentTheme)
    .pop();
    
  document.documentElement.setAttribute("data-theme", nextTheme ? nextTheme : "lightMode");

  console.log(`Switch to ${nextTheme}`);
}
