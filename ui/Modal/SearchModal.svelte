<script lang="ts">
  import { push } from "svelte-spa-router";
  import { createEventDispatcher } from "svelte";

  import { Icon, Input } from "../DesignSystem/Primitive";

  import { Remote, TrackToggle, Urn } from "../DesignSystem/Component";

  import * as path from "../src/path";

  import { request, updateUrn, validation } from "../src/search";
  import { ValidationStatus } from "../src/validation";

  let searchBar: HTMLDivElement,
    value = "";

  const dispatch = createEventDispatcher();

  const navigateToProject = () => {
    if ($validation.status !== ValidationStatus.Success) return;

    dispatch("hide");
    push(path.projectUntracked(value));
  };

  const onKeydown = (ev: KeyboardEvent) => {
    switch (ev.code) {
      case "Enter":
        navigateToProject();
        break;
      case "Escape":
        dispatch("hide");
        break;
    }
  };

  $: if (value && value.length > 0) {
    updateUrn({ urn: value });
  }

  $: showTrackingInfo = value.length > 0;
</script>

<style>
  .container {
    width: 26.25rem;
  }

  .search-bar {
    margin-bottom: 1rem;
    position: relative;
  }

  .tracking-info {
    background: var(--color-background);
    cursor: pointer;
    border-radius: 0.5rem;
    height: 0;
    overflow: hidden;
    transition: height 0.5s linear;
  }

  .showTrackingInfo {
    height: 13.5rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.25rem;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container">
  <div class="search-bar" bind:this={searchBar}>
    <Input.Text
      autofocus
      bind:value
      placeholder="Have a Radicle project ID?"
      showLeftItem
      style="height: 3rem;"
      inputStyle="border: none; border-radius: 0.5rem; height: 3rem;"
      hint="v">
      <div slot="left" style="display: flex;">
        <Icon.MagnifyingGlass />
      </div>
    </Input.Text>
  </div>

  <!-- TODO(sos): Once we determine how searching works, make sure this looks right
    if user changes urn
  -->
  <div
    class="tracking-info"
    class:showTrackingInfo
    on:click={navigateToProject}>
    <div style="padding: 2rem;">
      <Remote store={request} let:data={response}>
        <div class="header">
          <!-- TODO(sos): Are project names going to be part of the strings users share w/each other?
        e.g. my-new-project@hwd1yref4dqt66zart1a9gcpb3i4kfgfjbdc7pp1xdzgsnkhmdyxpkm3cxe -->
          <h3 style="color: var(--color-foreground-level-6);">
            my-new-project
          </h3>
          <TrackToggle />
        </div>

        <div style="display: flex; margin-bottom: 1rem;">
          <Urn
            urn={response.urn}
            notificationText="The project ID was copied to your clipboard"
            truncate />
        </div>

        <p style="color: var(--color-foreground-level-6);">
          You’re not following this project yet, so there’s nothing to show
          here. Follow it and you’ll be notified as soon as it’s available.
        </p>

        <div slot="error" let:error>
          <!-- TODO(sos): validation & other errors go here -->
          <p>{error && error.message}</p>
        </div>
      </Remote>
    </div>
  </div>
</div>
