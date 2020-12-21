<script lang="ts">
  import { Button } from "../Primitive";

  import type { ButtonVariant } from "../../src/style";
  import * as notification from "../../src/notification";

  export let title: string = "";
  export let variant: ButtonVariant = "primary";
  export let dataCy = "";
  export let onClick: () => Promise<void>;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let errorMessage: (error: any) => string;
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
      console.log(JSON.stringify(error));
      notification.error(errorMessage(error), true);
    } finally {
      running = false;
    }
  }
</script>

<style>
  .button-wrapper {
    margin-left: 7px;
  }

  .running {
    cursor: wait;
  }
</style>

<span class="button-wrapper" class:running data-cy={dataCy}>
  <Button disabled={disabled || running} {variant} on:click={userDidClick}>
    {title}
  </Button>
</span>
