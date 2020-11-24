<script>
  import { getContext, onMount } from "svelte";

  import {
    settings,
    seedValidation,
    addSeed,
    removeSeed,
    updateAppearance,
  } from "../src/session.ts";
  import { themeOptions } from "../src/settings.ts";
  import * as path from "../src/path.ts";
  import * as modal from "../src/modal.ts";
  import { getVersion } from "../src/ipc";

  import { Button, Icon, Input } from "../DesignSystem/Primitive";
  import {
    PeerId,
    SidebarLayout,
    SegmentedControl,
    StyledCopyable,
  } from "../DesignSystem/Component";

  const updateTheme = event =>
    updateAppearance({ ...$settings.appearance, theme: event.detail });

  let seedInputValue;

  const submitSeed = async () => {
    if (await addSeed(seedInputValue)) {
      seedInputValue = "";
    }
  };

  $: if (seedInputValue === "") {
    seedValidation.reset();
  }

  let version;

  onMount(async () => {
    version = await getVersion();
  });

  const session = getContext("session");
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 64px auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding) 4rem var(--content-padding);
  }

  section header {
    margin: 16px 0 24px 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 12px;
    display: flex;
    justify-content: space-between;
  }

  .section-item {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1rem;
    align-items: center;
    margin-bottom: 24px;
    padding: 0 12px;
  }

  .section-item-single {
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

<SidebarLayout dataCy="page">
  <div class="container">
    <div class="title">
      <h1>Settings</h1>
      <span
        class="typo-link"
        on:click|stopPropagation={() => modal.toggle(path.shortcuts())}>
        Keyboard shortcuts
      </span>
    </div>

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
              href="https://docs.radicle.xyz/docs/understanding-radicle/faq">Learn
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
        <form class="seed-entry-form" on:submit|preventDefault>
          <div class="seed-entry-field">
            <Input.Text
              hint="v"
              bind:value={seedInputValue}
              placeholder="Enter a seed address here"
              style="margin-right: 8px; min-width: 224px; width: 100%;"
              validation={$seedValidation} />
            <Button
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

    <section>
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
</SidebarLayout>
