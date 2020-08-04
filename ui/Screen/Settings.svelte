<script>
  import {
    clear,
    clearCache,
    parseSeedsInput,
    settings,
    updateAppearance,
    updateCoCo,
    updateRegistry,
  } from "../src/session.ts";
  import { networkOptions, themeOptions } from "../src/settings.ts";

  import { Button, Input } from "../DesignSystem/Primitive";
  import { SidebarLayout, SegmentedControl } from "../DesignSystem/Component";

  let seedInputValue = $settings.coco.seeds.join("\n");

  const updateNetwork = event =>
    updateRegistry({ ...$settings.registry, network: event.detail });

  const updateTheme = event =>
    updateAppearance({ ...$settings.appearance, theme: event.detail });

  const updateSeeds = event => {
    const seeds = parseSeedsInput(event.target.value);
    updateCoCo({ ...$settings.coco, seeds });
    seedInputValue = seeds.join("\n");
  };
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 64px auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  section header {
    margin: 16px 0 24px 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 12px;
    display: flex;
    justify-content: space-between;
  }

  section header a {
    color: var(--color-secondary);
    text-decoration: underline;
  }

  section header a:hover {
    color: var(--color-secondary-level-6);
  }

  .section-item {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1rem;
    align-items: center;
    margin-bottom: 24px;
    padding: 0 12px;
  }

  .info {
    flex: 1;
  }

  .action {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    margin-left: 16px;
  }
</style>

<SidebarLayout dataCy="page">
  <div class="container">
    <h1 style="margin-bottom: 32px;">Settings</h1>

    <section>
      <header>
        <h3>Version</h3>
      </header>
      <div class="section-item">
        <div class="info">
          <p style="color: var(--color-foreground-level-6);">
            Version 01.45.02
          </p>
        </div>
        <div class="action">
          <p style="color: var(--color-foreground-level-6);">
            There’s a new version of Radicle Upstream
          </p>
          <Button style="margin-left: 16px;">Update to Version 01.45.03</Button>
        </div>
      </div>
    </section>

    <section>
      <header>
        <h3>Appearance</h3>
      </header>
      <div class="section-item">
        <div class="info">
          <p class="bold">Theme</p>
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
        <h3>Seeds</h3>
        <!-- TODO(sos): link to actual docs abt seeds -->
        <a href="link/to/docs">Learn about seeds</a>
      </header>
      <div class="section-item" style="align-items: flex-start;">
        <div class="info">
          <p class="bold">
            Seeds help you see more projects and people on the network
          </p>
          <p
            style="color: var(--color-foreground-level-6); margin-bottom: 24px;">
            Have some seed addresses you’d like to join? Enter them here and new
            projects from the seeds you’re subscribed to will appear in the
            Discover page.
          </p>
        </div>
        <div class="action">
          <Input.Textarea
            style="flex: 1; height: 6rem;"
            bind:value={seedInputValue}
            on:change={updateSeeds}
            placeholder="Enter seeds here" />
        </div>
      </div>
    </section>

    <section>
      <header>
        <h3>Registry</h3>
      </header>
      <div class="section-item">
        <div class="info">
          <p class="bold">Network</p>
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
        <h3>Session management</h3>
      </header>
      <div class="section-item">
        <div class="info">
          <p class="bold">Clear local cache</p>
          <p style="color: var(--color-foreground-level-6);">
            Removes all locally-stored temporary data from your device.
          </p>
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
          <p class="bold">Clears all authentication data</p>
          <p style="color: var(--color-foreground-level-6);">
            This is similar to how logout works. You will have to create a new
            identity or restore your existing identity.
          </p>
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
        <h3>Legal</h3>
      </header>
      <div class="section-item">
        <div class="info">
          <p class="bold">Twemoji</p>
          <p style="color: var(--color-foreground-level-6);">
            Copyright 2020 Twitter, Inc and other contributors. Licensed under
            CC-BY 4.0.
          </p>
        </div>
      </div>
      <div class="section-item">
        <div class="info">
          <p class="bold">Inter</p>
          <p style="color: var(--color-foreground-level-6);">
            Inter font by Rasmus Andersson licensed under the SIL Open Font
            License 1.1.
          </p>
        </div>
      </div>
      <div class="section-item">
        <div class="info">
          <p class="bold">Source Code Pro</p>
          <p style="color: var(--color-foreground-level-6);">
            Source Code Pro font by Adobe Fonts distributed under the SIL Open
            Font License.
          </p>
        </div>
      </div>
    </section>

  </div>
</SidebarLayout>
