<script>
  import {
    settings,
    updateAppearance,
    updateRegistry,
  } from "../src/session.ts";
  import { networkOptions, themeOptions } from "../src/settings.ts";

  import Title from "../DesignSystem/Primitive/Title.svelte";

  import Sidebar from "../Layout/Sidebar.svelte";

  import Appearance from "./Settings/Appearance.svelte";
  import Developer from "./Settings/Developer.svelte";
  import Legal from "./Settings/Legal.svelte";
  import Registry from "./Settings/Registry.svelte";
  import Session from "./Settings/Session.svelte";
  import Version from "./Settings/Version.svelte";

  const updateNetwork = event =>
    updateRegistry({ ...$settings.registry, network: event.detail });

  const updateTheme = event =>
    updateAppearance({ ...$settings.appearance, theme: event.detail });
</script>

<style>
  main {
    max-width: var(--content-max-width);
    margin: 64px auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  main :global(section header) {
    margin: 16px 0 24px 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 12px;
  }
  main :global(section .item) {
    padding: 0 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }
  main :global(.action) {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-left: 16px;
  }
</style>

<Sidebar>
  <main>
    <Title variant="big">Settings</Title>

    <Version />
    <Appearance
      appearance={$settings.appearance}
      options={themeOptions}
      on:update={updateTheme} />
    <Registry
      network={$settings.registry.network}
      options={networkOptions}
      on:update={updateNetwork} />
    <Session />
    <Developer />
    <Legal />
  </main>
</Sidebar>
