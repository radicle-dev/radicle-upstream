<script lang="typescript">
  import { status, StatusType } from "ui/src/localPeer";
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

  const connectedPeerCount = (peers: {
    [peerId: string]: string[];
  }): string => {
    const count = Object.keys(peers).length;
    return peerCount(count);
  };

  const peerCount = (count: number) => {
    if (count === 1) {
      return "1 peer";
    } else {
      return `${count} peers`;
    }
  };

  let seedInputValue: string = "";
  let statusText: string = "";
  let statusFill: string = "";

  const submitSeed = async () => {
    if (await addSeed(seedInputValue)) {
      seedInputValue = "";
    }
  };

  $: if (seedInputValue === "") {
    seedValidation.reset();
  }

  $: {
    if ($status.type === StatusType.Online) {
      statusText = `You’re connected to ${connectedPeerCount(
        $status.connectedPeers
      )}`;
      statusFill = "var(--color-positive)";
    } else if ($status.type === StatusType.Syncing) {
      statusText = `Syncing with ${peerCount(
        $status.syncs
      )} to get new content from your network`;
      statusFill = "var(--color-caution)";
    } else if (
      $status.type === StatusType.Offline ||
      $status.type === StatusType.Started
    ) {
      statusText = "You’re not connected to any peers";
      statusFill = "var(--color-negative)";
    } else if ($status.type === StatusType.Stopped) {
      statusText = "The app couldn't start your peer";
      statusFill = "var(--color-negative)";
    }
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
          <circle cx="12" cy="12" r="4" fill={statusFill} />
        </svg>
        <p>{statusText}</p>
      </div>
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
