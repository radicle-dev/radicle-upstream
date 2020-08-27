<script>
  import { getContext } from "svelte";
  import { Wallet } from "../../DesignSystem/Component";

  const transactions = [];
  const session = getContext("session");
  $: accountId = session.identity ? session.identity.accountId : null;
  $: id = session.identity ? session.identity.metadata.handle : null;
</script>

<Wallet dataCy="user-wallet" balance={0} {transactions} {accountId} {id} />
