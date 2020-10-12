<script lang="ts">
  import { push } from "svelte-spa-router";
  import { createEventDispatcher } from "svelte";

  import { Icon, Input } from "../DesignSystem/Primitive";

  import {
    Remote,
    TrackToggle,
    ShareableIdentifier,
  } from "../DesignSystem/Component";

  import * as path from "../src/path";
  import * as remote from "../src/remote";

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

  let handle = "";

  $: if (value && value.length > 0) {
    updateUrn({ urn: value });
    handle = value.replace("rad:git:", "");
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
    height: 11rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .handle {
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
    color: var(--color-foreground-level-6);
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container">
  <div class="search-bar" bind:this={searchBar}>
    <Input.Text
      autofocus
      bind:value
      placeholder="Have a project handle? Paste it here…"
      showLeftItem
      style="height: 3rem;"
      inputStyle="border: none; border-radius: 0.5rem; height: 3rem; color: var(--color-foreground-level-6);"
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
    <div style="padding: 1.5rem;">
      <Remote store={request} let:data={response}>
        <div class="header typo-header-3">
          <span class="handle">{handle}</span>
          <TrackToggle style="margin-left: 1rem;" />
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
