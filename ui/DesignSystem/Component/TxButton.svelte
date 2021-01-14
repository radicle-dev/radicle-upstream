<script lang="ts">
  import { Button } from "../Primitive";

  import type { ButtonVariant } from "../../src/style";
  import * as notification from "../../src/notification";

  export let variant: ButtonVariant = "primary";
  export let dataCy = "";
  export let onClick: () => Promise<void>;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let errorLabel: string;
  export let disabled = false;

  let running = false;

  async function userDidClick(): Promise<void> {
    try {
      running = true;
      notification.info(
        "Waiting for you to confirm the transaction in your connected wallet.",
        true,
        false
      );
      await onClick();
    } catch (error) {
      notification.error(`${errorLabel}: ${error.message}`, true);
    } finally {
      running = false;
    }
  }
</script>

<style>
  .tx-button {
    margin-left: 0.4375rem;
  }

  .running {
    cursor: wait;
  }
</style>

<span class="tx-button" class:running data-cy={dataCy}>
  <Button disabled={disabled || running} {variant} on:click={userDidClick}>
    <slot />
  </Button>
</span>
