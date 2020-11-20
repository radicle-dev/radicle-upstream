<script lang="typescript">
  import TopUp from "../Outgoing/TopUp.svelte";

  import { wallet } from "../../../../../src/wallet";

  export let amount = 0;
  export let onBack: () => void;
  export let onContinue: () => void;

  $: accountBalance = $wallet.connected.account.balance * 1;
</script>

<TopUp
  bind:amount
  balance={accountBalance}
  onBack={['Back', onBack]}
  onContinue={['Continue', () => {
      onContinue();
      return Promise.resolve();
    }]} />
