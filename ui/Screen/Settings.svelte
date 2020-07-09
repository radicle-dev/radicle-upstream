<script>
  import {
    clear,
    clearCache,
    addSeed,
    removeSeed,
    seeds,
    settings,
    updateAppearance,
    updateRegistry,
  } from "../src/session.ts";
  import {
    networkOptions,
    seedValidation,
    themeOptions,
  } from "../src/settings.ts";
  import { ValidationStatus } from "../src/validation.ts";

  import { Button, Icon, Input, Text, Title } from "../DesignSystem/Primitive";
  import { SidebarLayout, SegmentedControl } from "../DesignSystem/Component";

  let seedInputValue;

  const updateNetwork = event =>
    updateRegistry({ ...$settings.registry, network: event.detail });

  const updateTheme = event =>
    updateAppearance({ ...$settings.appearance, theme: event.detail });

  const submitSeed = seed => {
    seedValidation.validate(seed);
    if ($seedValidation.status !== ValidationStatus.Success) return;

    addSeed(seed);
    seedInputValue = "";
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

  .info {
    flex: 1;
  }

  .action {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-left: 16px;
  }

  .seed-entry-form {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .seed-entry-field {
    display: flex;
  }

  .seeds {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    margin-top: 16px;
  }

  .seed {
    display: flex;
    align-items: center;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    margin: 0 0 8px 8px;
    padding: 8px;
    cursor: default;
    max-width: 100%;
  }

  .seed:hover {
    box-shadow: 0 0 0 1px
      var(--focus-outline-color, var(--color-foreground-level-3));
    background: var(--color-foreground-level-1);
  }
</style>

<SidebarLayout dataCy="page">
  <div class="container">
    <Title style="margin-bottom: 32px;" variant="huge">Settings</Title>

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
        <a href="link/to/docs">Learn about seeds</a>
      </header>
      <div class="section-item" style="align-items: flex-start;">
        <div class="info">
          <Text variant="medium">
            Seeds help you see more projects and people on the network
          </Text>
          <Text style="color: var(--color-foreground-level-6);">
            Have some seed addresses you’d like to join? Enter them here and new
            projects from the seeds you’re subscribed to will appear in the
            Discover page.
          </Text>
        </div>
        <form class="seed-entry-form">
          <div class="seed-entry-field">
            <Input.Text
              bind:value={seedInputValue}
              placeholder="Enter a seed address here"
              style="margin-right: 8px; min-width: 224px;"
              validation={$seedValidation} />
            <Button
              on:click={submitSeed(seedInputValue)}
              disabled={!seedInputValue}
              variant="outline">
              Add
            </Button>
          </div>

          <div class="seeds">
            {#each $seeds.data as seed}
              <div class="seed">
                <Icon.Cross
                  on:click={removeSeed(seed)}
                  variant="medium"
                  style="margin-right: 8px;" />

                <Text
                  class="setting-text"
                  style="color: var(--color-foreground-level-6);">
                  {seed}
                </Text>
              </div>
            {/each}
          </div>
        </form>
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
