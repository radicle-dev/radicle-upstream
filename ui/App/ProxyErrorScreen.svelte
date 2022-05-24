<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type * as svelteStore from "svelte/store";
  import type { SvelteComponent } from "svelte";

  import CheckIcon from "design-system/icons/Check.svelte";
  import CopyIcon from "design-system/icons/Copy.svelte";

  import Button from "design-system/Button.svelte";
  import Emoji from "design-system/Emoji.svelte";

  import * as error from "ui/src/error";
  import * as ipc from "ui/src/ipc";
  import * as notification from "ui/src/notification";

  // We have to circumvent the type checker because svelte cannot
  // narrow types using `if` statements.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const fatalError: svelteStore.Readable<any> = error.fatalError;

  let copied = false;
  let copyIcon: typeof SvelteComponent;
  $: copyIcon = copied ? CheckIcon : CopyIcon;

  function copyToClipboard(text: string): void {
    ipc.copyToClipboard(text);
    notification.show({ type: "info", message: "Copied to your clipboard" });
    copied = true;
    setTimeout(() => {
      copied = false;
    }, 2000);
  }

  function support(): void {
    ipc.openUrl(
      "https://discord.com/channels/841318878125490186/843873418205331506"
    );
  }
</script>

<style>
  p {
    margin-bottom: 1.5rem;
  }

  .container {
    background-color: var(--color-primary);
    height: 100vh;
    width: 100vw;
    position: fixed;
    z-index: 200;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: scroll;
  }

  .content {
    height: 100%;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding) 1rem;

    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    color: #fff; /* I know... but this design doesn't work in dark mode. */
  }

  .proxy-log-container {
    background: #e3e3ff;

    max-width: -webkit-fill-available;
    min-width: var(--content-min-width);
    max-height: 20vh;
    overflow: scroll;

    margin-top: 2rem;
    border-radius: 0.5rem;
    padding: 0.9em;
  }

  .proxy-log {
    display: block;
    color: #5555ff;
    font-size: 14px;
    white-space: pre-wrap;
    line-height: 1.4;
  }
</style>

{#if $fatalError !== null}
  <div class="container">
    <div class="content">
      <Emoji emoji="🧻" size="huge" style="margin-bottom: 1.5rem;" />
      <p style="width: 321px; text-align: center">
        {#if $fatalError.kind === "SESSION"}
          We're not totally sure what's going on, but we can't load the app
        {:else if $fatalError.kind === "PROXY_EXIT"}
          Hmm, looks like the app can’t be loaded right now because the backend
          has crashed or it isn’t starting.
        {/if}
      </p>
      <Button
        style="display: flex; background: #fff; color: var(--color-primary);"
        on:click={support}>
        Reach out for support
      </Button>
      {#if $fatalError.kind === "PROXY_EXIT" && $fatalError.data.output}
        <div class="proxy-log-container">
          <code class="proxy-log typo-mono-bold">
            {$fatalError.data.output}
          </code>
          <Button
            style="position: sticky; bottom: 0; margin-left: auto;"
            on:click={() => copyToClipboard($fatalError.data.output)}
            icon={copyIcon}>
            Copy to clipboard
          </Button>
        </div>
      {/if}
    </div>
  </div>
{/if}
