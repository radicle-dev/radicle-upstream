<script>
  import {
    clear,
    clearCache,
    fetchSeeds,
    settings,
    updateAppearance,
    updatePeerConfig,
    updateRegistry,
  } from "../src/session.ts";
  import { networkOptions, themeOptions } from "../src/settings.ts";

  import { Button, Input, Text, Title } from "../DesignSystem/Primitive";
  import { SidebarLayout, SegmentedControl } from "../DesignSystem/Component";

  let seeds = fetchSeeds();

  const updateNetwork = event =>
    updateRegistry({ ...$settings.registry, network: event.detail });

  const updateTheme = event =>
    updateAppearance({ ...$settings.appearance, theme: event.detail });

  const updateSeeds = event => {
    updatePeerConfig(event.target.value);
  };
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 64px 100px;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  section header {
    margin: 16px 0 24px 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 12px 0;
    display: flex;
    justify-content: space-between;
  }

  section header a {
    color: var(--color-secondary);
  }

  section header a:hover {
    text-decoration: underline;
  }

  .section-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }

  .action {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-left: 16px;
  }

  @media (max-width: 1000px) {
    .container {
      margin: 64px auto;
    }
  }
</style>

<SidebarLayout dataCy="page">
  <div class="container">
    <Title style="margin-bottom: 32px;" variant="big">Settings</Title>

    <section>
      <header>
        <Title variant="large">Version</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text style="color: var(--color-foreground-level-6);">
            Version 01.45.02
          </Text>
        </div>
        <div class="action">
          <Text style="color: var(--color-foreground-level-6);">
            There’s a new version of Radicle Upstream
          </Text>
          <Button style="margin-left: 16px;">Update to Version 01.45.03</Button>
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Appearance</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Theme</Text>
        </div>
        <div class="action">
          <SegmentedControl
            active={$settings.appearance.theme}
            options={themeOptions}
            on:select={updateTheme} />
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Seeds</Title>
        <!-- TODO(sos): link to actual docs abt seeds -->
        <!-- TODO(sos): should we have a submit button for seeds? -->
        <a href="link/to/docs">Learn about seeds</a>
      </header>
      <div class="info">
        <Input.Textarea bind:value={seeds} on:change={updateSeeds} />
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Registry</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Network</Text>
        </div>
        <div class="action">
          <SegmentedControl
            active={$settings.registry.network}
            options={networkOptions}
            on:select={updateNetwork} />
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Session management</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Clear local cache</Text>
          <Text style="color: var(--color-foreground-level-6);">
            Removes all locally-stored temporary data from your device.
          </Text>
        </div>
        <div class="action">
          <Button
            dataCy="clear-cache-button"
            variant="outline"
            on:click={clearCache}>
            Clear cache
          </Button>
        </div>
      </div>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Clears all authentication data</Text>
          <Text style="color: var(--color-foreground-level-6);">
            This is similar to how logout works. You will have to create a new
            identity or restore your existing identity.
          </Text>
        </div>
        <div class="action">
          <Button
            dataCy="clear-session-button"
            variant="outline"
            on:click={clear}>
            Clear session
          </Button>
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Legal</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Twemoji</Text>
          <Text style="color: var(--color-foreground-level-6);">
            Copyright 2020 Twitter, Inc and other contributors. Licensed under
            CC-BY 4.0.
          </Text>
        </div>
      </div>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Inter</Text>
          <Text style="color: var(--color-foreground-level-6);">
            Inter font by Rasmus Andersson licensed under the SIL Open Font
            License 1.1.
          </Text>
        </div>
      </div>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Source Code Pro</Text>
          <Text style="color: var(--color-foreground-level-6);">
            Source Code Pro font by Adobe Fonts distributed under the SIL Open
            Font License.
          </Text>
        </div>
      </div>
    </section>

  </div>
</SidebarLayout>
