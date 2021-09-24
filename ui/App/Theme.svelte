<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { theme, codeFont, uiFont, primaryColor } from "ui/src/appearance";

  $: {
    document.documentElement.setAttribute("data-theme", $theme);
    document.documentElement.setAttribute("data-uifont", $uiFont);
    document.documentElement.setAttribute("data-codefont", $codeFont);
    document.documentElement.setAttribute("data-primary-color", $primaryColor);
  }
</script>
