<script lang="typescript">
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

  let seedInputValue = "";

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
    align-items: center;
    margin-bottom: 24px;
    padding: 0 12px;
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

<SidebarLayout dataCy="network-page">
  <div class="container">
    <div class="title">
      <h1>Network</h1>
    </div>
    <section>
      <div class="info">
        <p class="typo-text-bold">
          Seeds help you find projects and users on the network.
        </p>
        <p style="color: var(--color-foreground-level-6); margin-bottom: 24px;">
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
    </section>
  </div>
</SidebarLayout>
