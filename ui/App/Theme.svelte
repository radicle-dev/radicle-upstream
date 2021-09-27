<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import {
    theme,
    codeFont,
    uiFont,
    primaryColor,
    primaryColorHex,
  } from "ui/src/appearance";

  $: document.documentElement.setAttribute("data-theme", $theme);
  $: document.documentElement.setAttribute("data-uifont", $uiFont);
  $: document.documentElement.setAttribute("data-codefont", $codeFont);
  $: document.documentElement.setAttribute("data-primary-color", $primaryColor);

  const customPrimaryColorRule = makeCssStyleRule();
  $: {
    const { r, g, b } = hexToRgbColor($primaryColorHex);
    customPrimaryColorRule.style.setProperty(
      "--color-primary",
      `rgba(${r},${g},${b},1)`
    );
  }

  function hexToRgbColor(hex: string): { r: number; g: number; b: number } {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result
      ? {
          r: parseInt(result[1], 16),
          g: parseInt(result[2], 16),
          b: parseInt(result[3], 16),
        }
      : {
          r: 21,
          g: 21,
          b: 21,
        };
  }

  function makeCssStyleRule(): CSSStyleRule {
    const element = document.createElement("style");
    document.head.appendChild(element);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const sheet = element.sheet!;
    sheet.insertRule(`[data-primary-color="custom"] {}`);
    return sheet.cssRules[0] as CSSStyleRule;
  }
</script>
