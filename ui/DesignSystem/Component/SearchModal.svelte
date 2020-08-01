<script>
  import { createEventDispatcher } from "svelte";

  import { Code, Icon, Input, Text } from "../Primitive";

  import Copyable from "./Copyable.svelte";
  import TrackToggle from "./TrackToggle.svelte";

  import { Status } from "../../src/remote";
  import { project, updateUri, validation } from "../../src/search";

  let searchBar,
    hide = false,
    value;

  const onKeydown = ev => {
    switch (ev.key) {
      case "Escape":
        hide = true;
        dispatch("hide");
        break;
      case "Enter":
        if ($project.status === Status.Success) {
          hide = true;
        }
        break;
    }
  };

  const dispatch = createEventDispatcher();

  const clickOutside = ev => {
    if (!hide && ev.target !== searchBar && !searchBar.contains(ev.target)) {
      hide = true;
      showTrackingInfo = false;
    }
  };

  $: if (value && value.length > 0) {
    updateUri(value);
  }

  // TODO(sos): animate & show/hide based on actual remote response
  $: showTrackingInfo =
    value && value.length > 0 && $project.status === Status.Success;
</script>

<style>
  .overlay {
    /* TODO(sos): ask brandonhaslegs for color & opacity for both light & dark modes */
    background-color: black;
    opacity: 0.7;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    z-index: 900;
  }

  .search-modal {
    /* TODO(sos): cute animation */
    display: flex;
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    z-index: 10000;

    align-items: center;
    justify-content: center;
    text-align: center;
  }
  .hide {
    display: none;
  }

  .content {
    z-index: 1000;
    width: 420px;
  }

  .search-bar {
    margin-bottom: 16px;
  }

  .tracking-info {
    background: var(--color-background);
    border-radius: 4px;
    padding: 32px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;

    margin-bottom: 48px;
  }

  .shareable-entity-identifier {
    background: var(--color-foreground-level-2);
    max-width: 180px;
    padding: 4px;
    border-radius: 4px;
  }
</style>

<svelte:window on:click={clickOutside} on:keydown={onKeydown} />

<div class="search-modal" class:hide>
  <div class="overlay" />
  <div class="content">
    <div class="search-bar" bind:this={searchBar}>
      <Input.Text
        autofocus
        bind:value
        placeholder="Have a Radicle project ID? Paste it here..."
        showLeftItem
        validation={$validation}>
        <div slot="left" style="display: flex;">
          <Icon.Search />
        </div>
      </Input.Text>
    </div>

    {#if showTrackingInfo}
      <div class="tracking-info">
        <div class="header">
          <div class="shareable-entity-identifier">
            <Copyable style="min-width: 0;">
              <Code
                variant="medium"
                style="font-size: 14px; color: var(--color-foreground-level-6);
                text-overflow: ellipsis; white-space: nowrap; overflow: hidden;">
                bshw82ienbytkx8173ndja0sjen833j88113jcb
              </Code>
            </Copyable>
          </div>

          <TrackToggle />
        </div>
        <Text style="text-align: left; color: var(--color-foreground-level-6);">
          You’re not tracking this project yet, so there’s nothing to show here.
          Track it and you’ll be notified as soon as it’s available.
        </Text>
      </div>
    {/if}
  </div>
</div>
