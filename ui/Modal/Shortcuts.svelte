<script>
  import { Illustration, KeyHint } from "../DesignSystem/Component";
  import { Variant as IllustrationVariant } from "../src/illustration";
  import { isDev } from "../../native/ipc.js";

  export let content;
</script>

<style>
  .container {
    width: 38.5rem;
    background: var(--color-background);
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
  }

  .shortcut {
    margin-bottom: 1rem;
    display: flex;
    align-items: center;
  }

  .shortcut:first-child {
    margin-top: 1.5rem;
  }

  .description {
    color: var(--color-foreground-level-6);
    margin: 0rem 0.625rem;
  }

  .container:focus {
    outline: none;
  }
</style>

<div data-cy="hotkey-modal" class="container" bind:this={content}>
  <Illustration
    style="margin-bottom: 1.5rem;"
    variant={IllustrationVariant.Keyboard} />
  <h1>Keyboard shortcuts</h1>
  <div class="shortcuts">
    <div class="shortcut">
      <KeyHint hint="?" />
      <p class="description">Keyboard shortcuts</p>
    </div>
    <div class="shortcut">
      <KeyHint modifier hint="," />
      <p class="description">Settings</p>
    </div>
    {#if isDev()}
      <div class="shortcut">
        <KeyHint modifier hint="d" />
        <p class="description">Design system</p>
      </div>
    {/if}
    <div class="shortcut">
      <KeyHint modifier hint="p" />
      <p class="description">Search</p>
    </div>
    <div class="shortcut">
      <KeyHint modifier hint="n" />
      <p class="description">New project</p>
    </div>
    <div class="shortcut">
      <KeyHint hint="esc" />
      <p class="description">Close modal</p>
    </div>
  </div>
</div>
