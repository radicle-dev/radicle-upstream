<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { settings } from "ui/src/session";
  import { Theme, UIFont, CodeFont } from "ui/src/settings";

  $: {
    const theme = $settings ? $settings.appearance.theme : Theme.Dark;
    const uiFont = $settings ? $settings.appearance.uiFont : UIFont.Inter;
    const codeFont = $settings
      ? $settings.appearance.codeFont
      : CodeFont.SourceCode;
    document.documentElement.setAttribute("data-theme", theme);
    document.documentElement.setAttribute("data-uifont", uiFont);
    document.documentElement.setAttribute("data-codefont", codeFont);
  }
</script>
