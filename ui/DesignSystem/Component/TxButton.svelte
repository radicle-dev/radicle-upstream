<script lang="ts">
  import { Button } from "../Primitive";

  import type { ButtonVariant } from "../../src/style";
  import * as notification from "../../src/notification";
  import Spinner from "./Spinner.svelte";

  export let title: string = "";
  export let variant: ButtonVariant = "primary";
  export let dataCy = "";
  export let onClick: () => Promise<void>;
  export let errorMessage: (error: any) => string;
  export let disabled = false;

  let running = false;
  // TODO(nuno): Better this message.
  const successMessage: string =
    "Transaction approved. Pending mining. Please, wait..";

  async function userDidClick(): Promise<void> {
    try {
      running = true;
      notification.info("Approve the transaction on your wallet app 📲");
      await onClick();
      // Waiting a moment here smoothes the UI.
      await continueAfter(0.4);
      notification.success(successMessage);
    } catch (error) {
      notification.error(errorMessage(error), true);
    } finally {
      running = false;
    }
  }

  function continueAfter(seconds: number): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve();
      }, seconds * 1000);
    });
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
