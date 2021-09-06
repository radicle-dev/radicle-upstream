<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import Big from "big.js";
  import * as ethereum from "ui/src/ethereum";

  import { Button } from "ui/DesignSystem";
  import TopUp from "../TopUp.svelte";

  import { accountBalancesStore } from "ui/src/wallet";

  export let amount = "";
  export let onBack: () => void;
  export let onContinue: () => void;

  let disabled = true;
  $: balance = ethereum.toBaseUnit($accountBalancesStore.dai || Big(0));

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter" && !disabled) {
      onContinue();
    }
  };
</script>

<svelte:window on:keydown={onKeydown} />

<TopUp bind:amount {balance} onBack={["Back", onBack]} bind:disabled>
  <Button on:click={onContinue} {disabled}>Continue</Button>
</TopUp>
