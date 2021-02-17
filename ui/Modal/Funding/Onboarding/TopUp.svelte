<script lang="typescript">
  import * as svelteStore from "svelte/store";

  import { Button } from "../../../DesignSystem/Primitive";
  import TopUp from "../../../DesignSystem/Component/Funding/Pool/TopUp.svelte";

  import { store as walletStore } from "../../../src/wallet";

  import Big from "big.js";

  export let amount = "";
  export let onBack: () => void;
  export let onContinue: () => void;

  let disabled = true;
  let accountBalance = Big(0);
  $: accountBalance =
    svelteStore.get(walletStore).account()?.balance || accountBalance;

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter" && !disabled) {
      onContinue();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />

<TopUp
  bind:amount
  balance={accountBalance}
  onBack={['Back', onBack]}
  bind:disabled>
  <Button on:click={onContinue} {disabled}>Continue</Button>
</TopUp>
