<script lang="typescript">
  import type { SvelteComponent } from "svelte";
  import type * as svelteStore from "svelte/store";

  import { Button, Emoji, Icon } from "../DesignSystem/Primitive";

  import * as notification from "../src/notification";
  import * as error from "../src/error";

  // We have to circumvent the type checker because svelte cannot
  // narrow types using `if` statements.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const fatalError: svelteStore.Readable<any> = error.fatalError;

  let copied = false;
  let copyIcon: typeof SvelteComponent;
  $: copyIcon = copied ? Icon.Check : Icon.Copy;

  const copyToClipboard = (text: string) => {
    // FIXME(rudolfs)
    // ipc.copyToClipboard(text);
    console.log(text);
    notification.info("Copied to your clipboard");
    copied = true;
    setTimeout(() => {
      copied = false;
    }, 2000);
  };

  const support = () => {
    window.location.href = "https://matrix.to/#/#support:radicle.community";
  };
</script>

<style>
  p {
    margin-bottom: 1.5rem;
  }

  .container {
    background-color: var(--color-secondary);
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
    padding: 5rem;

    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    color: #fff; /* I know... but this design doesn't work in dark mode. */
  }

  .proxy-log-container {
    background: #e3e3ff;

    max-width: 100%;
    min-width: 40em;
    max-height: 20vh;
    overflow: scroll;

    margin-top: 2rem;
    border-radius: 8px;
    padding: 0.9em;
  }

  .proxy-log {
    display: block;
    color: #5555ff;
    font-size: 14px;
    white-space: pre-wrap;
    line-height: 1.4;

    max-width: 100%;
  }
</style>

{#if $fatalError !== null}
  <div class="container">
    <div class="content" data-cy="blue-screen-of-death">
      <Emoji emoji="🧻" size="huge" style="margin-bottom: 1.5rem;" />
      <p style="width: 321px; text-align: center">
        {#if $fatalError.kind === 'SESSION'}
          We're not totally sure what's going on, but we can't load the app
        {:else if $fatalError.kind === 'PROXY_EXIT'}
          Hmm, looks like the app can’t be loaded right now because the backend
          has crashed or it isn’t starting.
        {/if}
      </p>
      <Button style="display: flex; color: #fff;" on:click={support}>
        Reach out for support
      </Button>
      {#if $fatalError.kind === 'PROXY_EXIT' && $fatalError.data.output}
        <div class="proxy-log-container">
          <code data-cy="proxy-log" class="proxy-log typo-mono-semi-bold">
            {$fatalError.data.output}
          </code>
          <Button
            dataCy="proxy-log-copy-clipboard"
            style="position: sticky; bottom: 0; margin-left: auto;"
            variant="secondary"
            on:click={() => copyToClipboard($fatalError.data.output)}
            icon={copyIcon}>
            Copy to clipboard
          </Button>
        </div>
      {/if}
    </div>
  </div>
{/if}
