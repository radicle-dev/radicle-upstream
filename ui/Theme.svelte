<script lang="ts">
  import { settings } from "./src/session";
  import { Theme } from "./src/settings";

  $: {
    const theme = $settings ? $settings.appearance.theme : Theme.Light;
    document.documentElement.setAttribute("data-theme", theme);
  }
</script>
