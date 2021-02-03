<script lang="typescript">
  import { Button, Emoji } from "../../../DesignSystem/Primitive";
  import { TxButton } from "../../../DesignSystem/Component";

  import Receivers from "../../../DesignSystem/Component/Funding/Pool/Receivers.svelte";

  import type { Receivers as PoolReceivers } from "../../../src/funding/pool";

  export let onBack: () => void;
  export let onConfirmed: () => Promise<void>;

  export let budget = "";
  export let topUp = "";
  export let receivers: PoolReceivers;
</script>

<style>
  h1,
  p,
  .submit {
    margin-top: 1.5rem;
  }

  h1,
  p {
    padding: 0 2.5rem;
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
  }

  strong {
    font-weight: bold;
  }
</style>

<Emoji emoji="💸" size="huge" />
<h1>Stream digital money</h1>
<p>
  {#if receivers.size === 0}
    Top up
    <strong>{topUp} DAI</strong>. You haven’t added any receivers yet, but as
    soon as you do, money will begin streaming to them at a rate of
    <strong>{budget} DAI</strong>
    per week.
  {:else}
    Top up
    <strong>{topUp} DAI</strong>
    and stream
    <strong>{budget} DAI</strong>
    per week to these users:
  {/if}
</p>
<Receivers {receivers} style="margin-top: 1.5rem" />
<div class="submit">
  <Button
    variant="transparent"
    dataCy="back"
    on:click={onBack}
    style="margin-right: 1rem">
    Back
  </Button>

  <TxButton
    dataCy="confirm-button"
    onClick={onConfirmed}
    errorLabel="Failed to onboard your pool">
    Confirm in your wallet
  </TxButton>
</div>
