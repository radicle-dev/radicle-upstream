<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as svelteStore from "svelte/store";

  import { status } from "ui/src/localPeer";
  import * as proxy from "ui/src/proxy";
  import { indicatorState } from "ui/src/network";
  import { VALID_SEED_MATCH } from "ui/src/session";

  import CrossIcon from "design-system/icons/Cross.svelte";

  import Button from "design-system/Button.svelte";
  import TextInput from "design-system/TextInput.svelte";
  import type { TextInputValidationState } from "design-system/TextInput";

  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  const indicatorStatus = svelteStore.derived(status, indicatorState);

  let seeds: string[] = [];
  let loaded = false;
  let seedInputValue: string = "";

  let seedInputValidationState: TextInputValidationState = {
    type: "unvalidated",
  };

  $: if (seedInputValue === "") {
    seedInputValidationState = { type: "unvalidated" };
  }

  fetchSeeds();

  async function addSeed() {
    const validationResult = validateSeed(seedInputValue);
    if (validationResult === true) {
      seedInputValidationState = { type: "valid" };
    } else {
      seedInputValidationState = { type: "invalid", message: validationResult };
      return;
    }

    await updateSeeds(seeds => [...seeds, seedInputValue]);
  }

  async function fetchSeeds() {
    seeds = await proxy.client.seedsGet();
    loaded = true;
  }

  function removeSeed(index: number) {
    updateSeeds(seeds => {
      seeds.splice(index, 1);
      return seeds;
    });
  }

  async function updateSeeds(f: (seeds: string[]) => string[]) {
    seeds = f(seeds);
    await proxy.client.seedsPut(seeds);
  }

  // Returns `true` if `seed` is a valid seed address and an error message
  // otherwise.
  function validateSeed(seed: string): true | string {
    if (!VALID_SEED_MATCH.exec(seed)) {
      return "This is not a valid seed address";
    }

    if (seeds.includes(seed)) {
      return "This seed already exists";
    }

    return true;
  }
</script>

<style>
  section {
    margin-bottom: 1.5rem;
    padding: 0 0.75rem;
  }

  .seed-entry-form {
    display: flex;
    flex-direction: column;
    margin-top: 1rem;
  }

  .seed-entry-field {
    width: 100%;
    display: flex;
    align-items: flex-start;
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
    margin: 2rem 0;
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

<ScreenLayout dataCy="network-page">
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
          validationState={seedInputValidationState} />
        <Button
          dataCy="add-seed"
          on:click={addSeed}
          disabled={!seedInputValue || !loaded}
          variant="outline">
          Add
        </Button>
      </div>
      <div class="seeds">
        {#each seeds as seed, index (seed)}
          <div class="seed">
            <CopyableIdentifier value={seed} kind="seedAddress" />
            <CrossIcon
              dataCy="remove-seed"
              on:click={() => removeSeed(index)}
              style="margin-left: 1.5rem; cursor:pointer;" />
          </div>
        {/each}
      </div>
    </form>
  </section>
</ScreenLayout>
