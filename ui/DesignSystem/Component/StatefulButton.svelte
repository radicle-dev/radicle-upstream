<script lang="ts">
  import { Button } from "../Primitive";

  import type { ButtonVariant } from "../../src/style";
  import * as notification from "../../src/notification";

  export let title: string = "";
  export let variant: ButtonVariant = "primary";
  export let dataCy = "";
  export let onClick: () => Promise<void>;
  export let successMessage: string = "Success";
  export let errorMessage: (error: any) => string;
  export let disabled = false;

  async function userDidClick(): Promise<void> {
    try {
      disabled = true;
      await onClick();
      // Waiting a moment here smoothes the UI.
      await continueAfter(0.4);
      notification.success(successMessage);
    } catch (error) {
      notification.error(errorMessage(error), true);
    } finally {
      disabled = false;
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

<span class="button-wrapper" data-cy={dataCy}>
  <Button {disabled} {variant} on:click={userDidClick}>{title}</Button>
</span>
