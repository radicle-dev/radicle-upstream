<script>
  import { onMount } from "svelte";

  import { clear, settings, updateAppearance } from "../src/session.ts";
  import {
    seedValidation,
    themeOptions,
    addSeed,
    removeSeed,
  } from "../src/settings.ts";
  import * as path from "../src/path.ts";
  import * as modal from "../src/modal.ts";
  import { getVersion, isDev } from "../../native/ipc.js";

  import { Button, Icon, Input } from "../DesignSystem/Primitive";
  import {
    SidebarLayout,
    SegmentedControl,
    Urn,
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
          <p class="typo-text-bold">Join the developer chat</p>
        </div>
        <div class="action">
          <a class="typo-link" href="irc://freenode:1/radicle">
            #radicle on freenode
          </a>
        </div>
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
        <h3>Network</h3>
      </header>
      <div class="section-item-single">
        <div class="info">
          <p class="typo-text-bold">
            Seeds help you see more projects and people on the network
          </p>
          <!-- TODO(julien): link to actual docs abt seeds -->
          <p
            style="color: var(--color-foreground-level-6); margin-bottom: 24px;">
            Enter seed addresses that you’d like to subscribe to here. <a style="color: var(--color-foreground-level-5);" class="typo-link" href="https://radicle.xyz/#upstream-faq"> Learn
              more about seeds </a>
          </p>
        </div>
        <form class="seed-entry-form" on:submit|preventDefault>
          <div class="seed-entry-field">
            <Input.Text
              hint="v"
              bind:value={seedInputValue}
              placeholder="Paste seed address here"
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
                <Urn urn={seed} />
                <Icon.Cross
                  on:click={() => removeSeed(seed)}
                  variant="medium"
                  style="margin-left: 1.5rem; cursor:pointer;" />
              </div>
            {/each}
          </div>
        </form>
      </div>
    </section>

    {#if isDev()}
      <section>
        <header>
          <h3>Session management</h3>
        </header>
        <div class="section-item">
          <div class="info">
            <p class="typo-text-bold">Clears all authentication data</p>
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
    {/if}

    <section>
      <header>
        <h3>Version</h3>
      </header>
      <div class="section-item">
        <div class="info">
          <p style="color: var(--color-foreground-level-6);">
            Version {version}
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
