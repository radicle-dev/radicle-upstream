<script>
  import { pop } from "svelte-spa-router";

  import { session } from "../../src/session.ts";
  import { fallback } from "../../src/identity.ts";
  import * as remote from "../../src/remote.ts";

  import { Button, Flex } from "../../DesignSystem/Primitive";
  import { Transaction } from "../../DesignSystem/Component";

  const onSubmitTransaction = () => {
    pop();
  };

  let identity = fallback;
  let handle = null;

  if (
    $session.status === remote.Status.Success &&
    $session.data.identity !== null
  ) {
    identity = $session.data.identity;
    handle = $session.data.identity.metadata.handle;
  }

  // TODO(rudolfs): move wallet selection to top-level component
  const transaction = {
    message: "Project registration",
    stake: "Project registration deposit",
    subject: {
      name: "project-name-goes-here",
      kind: "project",
      avatarFallback: identity.avatarFallback,
      imageUrl: identity.avatarUrl
    },
    payer: {
      name: handle,
      kind: "user",
      avatarFallback: identity.avatarFallback,
      imageUrl: identity.metadata.avatarUrl
    }
  };
</script>

<Transaction {transaction} />

<Flex style="margin-top: 32px;" align="right">
  <Button
    dataCy="cancel-button"
    variant="transparent"
    on:click={pop}
    style="margin-right: 24px;">
    Cancel
  </Button>
  <Button dataCy="next-button" on:click={onSubmitTransaction} variant="primary">
    Submit transaction
  </Button>
</Flex>
