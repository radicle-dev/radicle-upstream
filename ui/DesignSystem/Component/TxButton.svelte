<script lang="ts">
  import { Button } from "../Primitive";

  import type { ButtonVariant } from "../../src/style";
  import * as notification from "../../src/notification";
  import Spinner from "./Spinner.svelte";

  export let title: string = "";
  export let variant: ButtonVariant = "primary";
  export let dataCy = "";
  export let onClick: () => Promise<void>;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let errorMessage: (error: any) => string;
  export let disabled = false;
  export let showNotification = true;

  let running = false;

  async function userDidClick(): Promise<void> {
    try {
      running = true;
      if (showNotification) {
        notification.info("Approve the transaction on your wallet app 📲");
      }
      await onClick();
    } catch (error) {
      if (showNotification) {
        notification.error(errorMessage(error), true);
      }
    } finally {
      running = false;
    }
  }
</script>

<style>
  .button-wrapper {
    margin-left: 7px;
  }
</style>

<span class="button-wrapper" data-cy={dataCy}>
  {#if running}
    <Spinner />
  {:else}
    <Button {disabled} {variant} on:click={userDidClick}>{title}</Button>
  {/if}
</span>
