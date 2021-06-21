<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as svelteStore from "svelte/store";

  import { Button } from "ui/DesignSystem";
  import TopUp from "ui/DesignSystem/Funding/Pool/TopUp.svelte";

  import { store as walletStore } from "../../../src/wallet";

  import Big from "big.js";

  export let amount = "";
  export let onBack: () => void;
  export let onContinue: () => void;

  let disabled = true;
  let accountBalance = Big(0);
  $: accountBalance =
    svelteStore.get(walletStore).account()?.daiBalance || accountBalance;

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
  onBack={["Back", onBack]}
  bind:disabled>
  <Button on:click={onContinue} {disabled}>Continue</Button>
</TopUp>
