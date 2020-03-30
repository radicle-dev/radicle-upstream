// TODO(rudolfs): fetch colors from Figma
// https://github.com/radicle-dev/radicle-upstream/issues/241
import colorTokens from "../tokens/colors.json";
import fs from "fs";
import path from "path";

let colorCss = `/* This file is auto-generated via \`yarn generate:colors\`, don't edit this
 * file manually. If you have to make changes to the color tokens edit
 * \`tokens/colors.json\` and re-generate this file by running the yarn script. */

:root {
`;

colorTokens.map(color => {
  colorCss += `  --color-${color.name}: ${color.hex};\n`;
});

colorCss += "}\n";

const pathToFile = path.resolve(__dirname, "../public/colors.css");

fs.writeFile(pathToFile, colorCss, error => {
  if (error) {
    return console.log(error);
  }

  console.log(
    `Generated ${pathToFile}, please remember to commit the file to the repo.`
  );
});
