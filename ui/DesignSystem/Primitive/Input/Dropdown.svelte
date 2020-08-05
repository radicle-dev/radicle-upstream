<script>
  import { createEventDispatcher } from "svelte";

  export let items = null;
  export let value = null;
  export let disabled = null;

  export let style = null;
  export let dataCy = null;

  const dispatch = createEventDispatcher();
  const change = () => dispatch("select", value);
</script>

<style>
  select {
    -moz-appearance: none;
    -webkit-appearance: none;
    appearance: none;
    height: 40px;
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 4px;
    color: var(--color-foreground-level-6);
    background: var(--color-background)
      url('data:image/svg+xml;utf8,<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M17.6247 14.7809C18.0559 14.4359 18.1258 13.8066 17.7808 13.3753C17.4358 12.944 16.8065 12.8741 16.3753 13.2191L12 16.7194L7.62466 13.2191C7.1934 12.8741 6.5641 12.944 6.21909 13.3753C5.87408 13.8066 5.94401 14.4359 6.37527 14.7809L11.3753 18.7809C11.7405 19.073 12.2594 19.073 12.6247 18.7809L17.6247 14.7809ZM16.3753 10.7809C16.8065 11.1259 17.4358 11.056 17.7808 10.6247C18.1258 10.1934 18.0559 9.56414 17.6247 9.21913L12.6247 5.21913C12.2594 4.92696 11.7405 4.92696 11.3753 5.21913L6.37527 9.21913C5.94401 9.56414 5.87408 10.1934 6.21909 10.6247C6.5641 11.056 7.1934 11.1259 7.62466 10.7809L12 7.28062L16.3753 10.7809Z" fill="%2390a0af"/></svg>')
      no-repeat;
    background-position: right 8px top 50%;
    padding: 2px 40px 2px 12px;
  }

  select:hover {
    outline: none;
    box-shadow: 0 0 0 1px
      var(--focus-outline-color, var(--color-foreground-level-3));
    border: 1px solid
      var(--focus-outline-color, var(--color-foreground-level-3));
    background-color: var(--color-foreground-level-1);
  }
  select:focus {
    outline: none;
  }

  select:disabled {
    color: var(--color-foreground-level-4);
    box-shadow: 0 0 0 0px;
    border: 1px solid var(--color-foreground-level-2);
    background: var(--color-background)
      url('data:image/svg+xml;utf8,<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" clip-rule="evenodd" d="M17.6247 14.7809C18.0559 14.4359 18.1258 13.8066 17.7808 13.3753C17.4358 12.944 16.8065 12.8741 16.3753 13.2191L12 16.7194L7.62466 13.2191C7.1934 12.8741 6.5641 12.944 6.21909 13.3753C5.87408 13.8066 5.94401 14.4359 6.37527 14.7809L11.3753 18.7809C11.7405 19.073 12.2594 19.073 12.6247 18.7809L17.6247 14.7809ZM16.3753 10.7809C16.8065 11.1259 17.4358 11.056 17.7808 10.6247C18.1258 10.1934 18.0559 9.56414 17.6247 9.21913L12.6247 5.21913C12.2594 4.92696 11.7405 4.92696 11.3753 5.21913L6.37527 9.21913C5.94401 9.56414 5.87408 10.1934 6.21909 10.6247C6.5641 11.056 7.1934 11.1259 7.62466 10.7809L12 7.28062L16.3753 10.7809Z" fill="%23C5D1DB"/></svg>')
      no-repeat;
    background-position: right 8px top 50%;
  }
</style>

<select
  class="typo-regular"
  data-cy={dataCy}
  {style}
  bind:value
  {disabled}
  on:blur={change}>
  {#each items as item}
    <option value={item}>{item}</option>
  {/each}
</select>
