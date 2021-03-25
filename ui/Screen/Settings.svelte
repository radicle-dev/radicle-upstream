<script lang="typescript">
  import { getContext } from "svelte";
  import * as svelteStore from "svelte/store";

  import { selectedEnvironment as ethereumEnvironment } from "../src/ethereum";
  import * as ethereum from "../src/ethereum";
  import * as ipc from "../src/ipc";
  import * as config from "../src/config";
  import {
    settings,
    seedValidation,
    addSeed,
    removeSeed,
    updateAppearance,
    updateFeatureFlags,
  } from "../src/session";
  import type { UnsealedSession } from "../src/session";
  import {
    themeOptions,
    featureFlagOptions,
    fundingEnvironmentOptions,
  } from "../src/settings";
  import { updateChecker } from "../src/updateChecker";
  import * as path from "../src/path";
  import * as modal from "../src/modal";

  import { Button, Icon, Input } from "../DesignSystem/Primitive";
  import {
    PeerId,
    SidebarLayout,
    SegmentedControl,
    StyledCopyable,
  } from "../DesignSystem/Component";

  const updateTheme = (event: CustomEvent) =>
    updateAppearance({ ...$settings.appearance, theme: event.detail });

  const updateFundingFeatureFlag = (event: CustomEvent) =>
    updateFeatureFlags({ ...$settings.featureFlags, funding: event.detail });

  const updateEthereumEnvironment = (event: CustomEvent) => {
    const environment = event.detail as ethereum.Environment;
    ethereum.selectedEnvironment.set(environment);
  };

  let seedInputValue = "";

  const submitSeed = async () => {
    if (await addSeed(seedInputValue)) {
      seedInputValue = "";
    }
  };

  $: if (seedInputValue === "") {
    seedValidation.reset();
  }

  let version = "";
  (async () => {
    version = await ipc.getVersion();
  })();

  // We trick TypeScript because svelte cannot deal with type refinement
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const latestVersionInfo: svelteStore.Readable<any> = updateChecker.newVersion();

  // This value is not reactive on purpose. We only want to move this to
  // the top on the initial render. Subsequent changes should not mess
  // with the layout as to not confuse the user.
  const showVersionAtTop = Boolean($latestVersionInfo);

  const appUpdateNotificationEnabled = svelteStore.derived(
    updateChecker.isEnabled(),
    isEnabled => (isEnabled ? "on" : "off")
  );

  const setAppUpdateNotificationEnabled = (event: CustomEvent) => {
    if (event.detail === "on") {
      updateChecker.enable();
    } else {
      updateChecker.disable();
    }
  };

  const appUpdateNotificationEnabledOptions = [
    { value: "on", title: "Notify Me" },
    { value: "off", title: "Turn off" },
  ];

  const session = getContext("session") as UnsealedSession;
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 64px auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  .sections {
    display: flex;
    flex-direction: column;
  }

  section header {
    margin: 16px 0 24px 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 12px;
    display: flex;
    justify-content: space-between;
  }

  .section-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
    padding: 0 12px;
  }

  .section-item-single {
    align-items: center;
    margin-bottom: 24px;
    padding: 0 12px;
  }

  .action {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    margin-left: 16px;
  }

  .seed-entry-form {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .seed-entry-field {
    width: 100%;
    display: flex;
    align-items: flex-start;
  }

  .seeds {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    margin-top: 1.5rem;
    width: 100%;
  }

  .seed {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-top: 1px solid var(--color-foreground-level-2);
    padding: 1.5rem 1rem 1.5rem 1rem;
    cursor: default;
  }

  .seed:last-of-type {
    border-bottom: 1px solid var(--color-foreground-level-2);
  }

  .title {
    display: flex;
    justify-content: space-between;
    margin-bottom: 32px;
    align-items: flex-end;
    padding: 0 0.75rem;
  }
</style>

<SidebarLayout dataCy="settings-page">
  <div class="container">
    <div class="title">
      <h1>Settings</h1>
      <span
        class="typo-link"
        on:click|stopPropagation={() => modal.toggle(path.shortcuts())}>
        Keyboard shortcuts
      </span>
    </div>
    <div class="sections">
      <section>
        <header>
          <h3>Devices</h3>
        </header>
        <div class="section-item">
          <div class="info">
            <p>
              Share your Device ID with others to be added as a remote.
              <br /><a
                style="color: var(--color-foreground-level-5);"
                class="typo-link"
                href="https://docs.radicle.xyz/docs/understanding-radicle/faq#can-i-use-radicle-with-multiple-devices">Learn
                more about managing devices</a>
            </p>
          </div>
          <div class="action">
            <PeerId peerId={session.identity.peerId} />
          </div>
        </div>
      </section>

      <section>
        <header>
          <h3>Network</h3>
        </header>
        <div class="section-item-single">
          <div class="info">
            <p class="typo-text-bold">
              Seeds help you find more projects and users on the network.
            </p>
            <p
              style="color: var(--color-foreground-level-6); margin-bottom: 24px;">
              Enter seed addresses that you’d like to connect to here.
              <a
                style="color: var(--color-foreground-level-5);"
                class="typo-link"
                href="https://docs.radicle.xyz/docs/understanding-radicle/glossary#seed">Learn
                more about seeds</a>
            </p>
          </div>
          <form
            class="seed-entry-form"
            on:submit|preventDefault
            data-cy="seed-entry-form">
            <div class="seed-entry-field">
              <Input.Text
                dataCy="seed-input"
                bind:value={seedInputValue}
                placeholder="Enter a seed address here"
                style="margin-right: 8px; min-width: 224px; width: 100%;"
                validation={$seedValidation} />
              <Button
                dataCy="add-seed"
                style="display: flex;"
                on:click={submitSeed}
                disabled={!seedInputValue}
                variant="outline">
                Add
              </Button>
            </div>

            <div class="seeds">
              {#each $settings.coco.seeds as seed (seed)}
                <div class="seed">
                  <StyledCopyable value={seed} />
                  <Icon.Cross
                    dataCy="remove-seed"
                    on:click={() => removeSeed(seed)}
                    style="margin-left: 1.5rem; cursor:pointer;" />
                </div>
              {/each}
            </div>
          </form>
        </div>
      </section>

      <section>
        <header>
          <h3>Appearance</h3>
        </header>
        <div class="section-item">
          <div class="info">
            <p class="typo-text-bold">Theme</p>
          </div>
          <div class="action">
            <SegmentedControl
              active={$settings.appearance.theme}
              options={themeOptions}
              on:select={updateTheme} />
          </div>
        </div>
      </section>

      {#if config.isExperimental}
        <section>
          <header>
            <h3>Features</h3>
          </header>
          <div class="section-item">
            <div class="info">
              <p class="typo-text-bold">Funding</p>
            </div>
            <div class="action">
              <SegmentedControl
                active={$settings.featureFlags.funding}
                options={featureFlagOptions}
                on:select={updateFundingFeatureFlag} />
            </div>
          </div>
          {#if $settings.featureFlags.funding}
            <div class="section-item">
              <div class="info">
                <p class="typo-text-bold">Funding environment</p>
              </div>
              <div class="action">
                <SegmentedControl
                  active={$ethereumEnvironment}
                  options={fundingEnvironmentOptions}
                  on:select={updateEthereumEnvironment} />
              </div>
            </div>
          {/if}
        </section>
      {/if}

      <section>
        <header>
          <h3>Feedback</h3>
        </header>
        <div class="section-item">
          <div class="info">
            <p class="typo-text-bold">Get in touch directly</p>
          </div>
          <div class="action">
            <a
              class="typo-link"
              href="https://radicle.community/c/site-feedback/2">
              radicle.community
            </a>
          </div>
        </div>
        <div class="section-item">
          <div class="info">
            <p class="typo-text-bold">Join the community chat</p>
          </div>
          <div class="action">
            <a class="typo-link" href="https://matrix.radicle.community">
              matrix.radicle.community
            </a>
          </div>
        </div>
      </section>

      <section data-cy="version" style={showVersionAtTop ? 'order: -1' : ''}>
        <header>
          <h3>Version</h3>
        </header>
        <div class="section-item">
          <div class="info">
            <p style="color: var(--color-foreground-level-6);">
              Version
              {version}
            </p>
          </div>
          {#if $latestVersionInfo}
            <div class="action">
              There’s a new version of Radicle Upstream
              <Button
                style="margin-left: 1em"
                dataCy="checkout-new-version"
                on:click={() => ipc.openUrl($latestVersionInfo.announcementUrl)}>
                Check out Version
                {$latestVersionInfo.version}
              </Button>
            </div>
          {/if}
        </div>
        <div class="section-item">
          <div class="info">
            Notification (Allow Upstream to make requests to the web)
          </div>
          <div class="action">
            <SegmentedControl
              active={$appUpdateNotificationEnabled}
              on:select={setAppUpdateNotificationEnabled}
              options={appUpdateNotificationEnabledOptions} />
          </div>
        </div>
      </section>

      <section>
        <header>
          <h3>Legal</h3>
        </header>
        <div class="section-item">
          <div class="info">
            <p class="typo-text-bold">Twemoji</p>
            <p style="color: var(--color-foreground-level-6);">
              Copyright 2020 Twitter, Inc and other contributors. Licensed under
              CC-BY 4.0.
            </p>
          </div>
        </div>
        <div class="section-item">
          <div class="info">
            <p class="typo-text-bold">Inter</p>
            <p style="color: var(--color-foreground-level-6);">
              Inter font by Rasmus Andersson licensed under the SIL Open Font
              License 1.1.
            </p>
          </div>
        </div>
        <div class="section-item">
          <div class="info">
            <p class="typo-text-bold">Source Code Pro</p>
            <p style="color: var(--color-foreground-level-6);">
              Source Code Pro font by Adobe Fonts distributed under the SIL Open
              Font License.
            </p>
          </div>
        </div>
      </section>
    </div>
  </div>
</SidebarLayout>
