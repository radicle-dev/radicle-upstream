import Color from "color";
import colorTokens from "../tokens/colors.json";
import fs from "fs";
import path from "path";

const generateTints = (color, range) => {
  return range.map(tint => {
    return `  --color-${color.name}-tint-${tint * 100}: ${Color(color.hex)
      .lighten(tint)
      .hex()
      .toLowerCase()};\n`;
  });
};

const generateShades = (color, range) => {
  return range.map(shade => {
    return `  --color-${color.name}-shade-${shade * 100}: ${Color(color.hex)
      .darken(shade)
      .hex()
      .toLowerCase()};\n`;
  });
};

let colorCss = `
/* This file is auto-generated via \`yarn generate:colors\`, don't edit this
 * file manually. If you have to make changes to the color tokens edit
 * \`tokens/colors.json\` and re-generate this file by running the yarn script. */

:root {
`;

colorTokens.map(color => {
  const tints = generateTints(color, [0.1]);

  const primary = `  --color-${color.name}: ${color.hex};\n`;

  const shades = generateShades(color, [0.1]);

  colorCss += tints + primary + shades;
});

colorCss += "}";

const pathToFile = path.resolve(__dirname, "../public/colors.css");

fs.writeFile(pathToFile, colorCss, error => {
  if (error) {
    return console.log(error);
  }

  console.log(
    `Generated ${pathToFile}, please remember to commit the file to the repo.`
  );
});
