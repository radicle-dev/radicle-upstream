<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as svelteStore from "svelte/store";

  import { status } from "ui/src/localPeer";
  import { indicatorState } from "ui/src/network";
  import {
    settings,
    seedValidation,
    addSeed,
    removeSeed,
  } from "../src/session";

  import {
    Button,
    Icon,
    SidebarLayout,
    StyledCopyable,
    TextInput,
  } from "ui/DesignSystem";

  const indicatorStatus = svelteStore.derived(status, indicatorState);

  let seedInputValue: string = "";

  const submitSeed = async () => {
    if (await addSeed(seedInputValue)) {
      seedInputValue = "";
    }
  };

  $: if (seedInputValue === "") {
    seedValidation.reset();
  }
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 64px auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  section {
    margin-bottom: 24px;
    padding: 0 12px;
  }

  .seed-entry-form {
    display: flex;
    flex-direction: column;
    margin-top: 1rem;
  }

  .seed-entry-field {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .seeds {
    display: flex;
    flex-direction: column;
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
    margin-bottom: 2rem;
    align-items: flex-end;
    padding: 0 0.75rem;
  }

  .status {
    display: flex;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 0.5rem;
  }
</style>

<SidebarLayout dataCy="network-page">
  <div class="container">
    <div class="title">
      <h1>Network</h1>
      <div class="status">
        <svg
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg">
          <circle cx="12" cy="12" r="4" fill={$indicatorStatus.fill} />
        </svg>
        <p>{$indicatorStatus.text}</p>
      </div>
    </div>
    <section>
      <div class="info">
        <p class="typo-text-bold">
          Seeds help you find more projects and people on the network.
        </p>
        <p style="color: var(--color-foreground-level-6);">
          Enter seed addresses that you’d like to connect to here.
          <a
            style="color: var(--color-foreground-level-5);"
            class="typo-link"
            href="https://docs.radicle.xyz/docs/understanding-radicle/glossary#seed"
            >Learn more</a>
        </p>
      </div>
      <form
        class="seed-entry-form"
        on:submit|preventDefault
        data-cy="seed-entry-form">
        <div class="seed-entry-field">
          <TextInput
            dataCy="seed-input"
            bind:value={seedInputValue}
            placeholder="Enter a seed address here"
            style="min-width: 14rem; width: 100%;"
            validation={$seedValidation} />
          <Button
            dataCy="add-seed"
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
    </section>
  </div>
</SidebarLayout>
