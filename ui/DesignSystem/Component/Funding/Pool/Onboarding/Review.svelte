<script lang="typescript">
  import { Button } from "../../../../Primitive";
  import { Illustration, TxButton } from "../../../../Component";

  import Receivers from "../Receivers.svelte";

  import { Variant as IllustrationVariant } from "../../../../../src/illustration";
  import * as pool from "../../../../../src/funding/pool";

  export let onBack: () => void;
  export let onConfirmed: () => Promise<void>;

  export let budget = 0;
  export let topUp = 0;
  export let receivers: pool.Receivers;
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

<Illustration variant={IllustrationVariant.Money} />
<h1>Stream digital money</h1>
<p>
  {#if receivers.size === 0}
    Top up
    <strong>{topUp} DAI</strong>. You haven’t added any receivers yet, but as
    soon as you do, money will begin streaming to them at a rate of
    <strong>{budget} DAI</strong>
    per month.
  {:else}
    Top up
    <strong>{topUp} DAI</strong>
    and stream
    <strong>{budget} DAI</strong>
    per month to these users:
  {/if}
</p>
<Receivers {receivers} />
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
    errorMessage={e => `Failed to onboard your pool: ${e.message}`}
    title={'Confirm in your wallet'} />
</div>
