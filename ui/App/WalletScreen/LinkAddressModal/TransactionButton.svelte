<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import Button from "design-system/Button.svelte";

  import * as notification from "ui/src/notification";
  import * as transaction from "ui/src/transaction";

  export let style: string | undefined = undefined;
  export let dataCy: string | undefined = undefined;
  export let onClick: () => Promise<void>;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let errorLabel: string;
  export let disabled = false;
  export let variant: "primary" | "destructive" = "primary";

  let running = false;

  async function userDidClick(): Promise<void> {
    try {
      running = true;
      notification.show({
        type: "info",
        message:
          "Waiting for you to confirm the transaction in your connected wallet",
      });
      await onClick();
    } catch (e: unknown) {
      notification.showException(transaction.convertError(e, errorLabel));
    } finally {
      running = false;
    }
  }
</script>

<style>
  .running {
    cursor: wait;
  }
</style>

<span class:running data-cy={dataCy} {style}>
  <Button disabled={disabled || running} {variant} on:click={userDidClick}>
    <slot />
  </Button>
</span>
