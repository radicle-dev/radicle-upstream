<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Button from "./Button.svelte";

  import type { ButtonVariant } from "ui/src/style";

  import * as notification from "ui/src/notification";
  import * as error from "ui/src/error";
  import * as transaction from "ui/src/transaction";

  export let variant: ButtonVariant = "primary";
  export let style = "";
  export let dataCy = "";
  export let onClick: () => Promise<void>;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  export let errorLabel: string;
  export let disabled = false;

  let running = false;

  async function userDidClick(): Promise<void> {
    try {
      running = true;
      notification.info({
        message:
          "Waiting for you to confirm the transaction in your connected wallet.",
        showIcon: true,
      });
      await onClick();
    } catch (e) {
      error.show(transaction.convertError(e, errorLabel));
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

<span class="tx-button" class:running data-cy={dataCy} {style}>
  <Button disabled={disabled || running} {variant} on:click={userDidClick}>
    <slot />
  </Button>
</span>
