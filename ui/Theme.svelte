<script>
  import { settings } from "./src/session.ts";
  import { Theme } from "./src/settings.ts";

  $: {
    const theme = $settings ? $settings.appearance.theme : Theme.Light;
    document.documentElement.setAttribute("data-theme", theme);
  }
</script>
