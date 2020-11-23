<script lang="typescript">
  import { Button } from "../../../../Primitive";
  import TopUp from "../Outgoing/TopUp.svelte";

  import { wallet } from "../../../../../src/wallet";

  export let amount = 0;
  export let onBack: () => void;
  export let onContinue: () => void;

  let disabled = true;

  $: accountBalance = $wallet.connected.account.balance * 1;
</script>

<TopUp
  bind:amount
  balance={accountBalance}
  onBack={['Back', onBack]}
  bind:disabled>
  <Button on:click={onContinue} {disabled}>Continue</Button>
</TopUp>
