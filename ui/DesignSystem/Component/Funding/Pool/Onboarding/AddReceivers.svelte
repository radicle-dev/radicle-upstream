<script lang="typescript">
  import { Button } from "../../../../Primitive";
  import { Illustration } from "../../../../Component";

  import Receivers from "../Receivers.svelte";
  import { Variant as IllustrationVariant } from "../../../../../src/illustration";
  import * as pool from "../../../../../src/funding/pool";

  export let receivers: pool.Receivers;
  export let onBack: () => void;
  export let onContinue: () => void;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    min-height: 500px;
  }

  h1,
  p {
    padding: 0 var(--content-padding);
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: calc(var(--content-padding) / 2);
  }
</style>

<div class="wrapper">
  <Illustration variant={IllustrationVariant.Money} />
  <h1>Add receivers</h1>
  <p>
    Add receivers to your outgoing support by clicking the “Support” button on
    profiles or project pages.
  </p>
  <p class="typo-text-small" style="color: var(--color-foreground-level-5)">
    Tip: You can also add new receivers later on by clicking on “Support” on
    their profile.
  </p>
  <Receivers bind:receivers editing={true} />
  <div class="submit">
    <Button
      variant="transparent"
      dataCy="cancel"
      on:click={onBack}
      style="margin-right: 1rem">
      Back
    </Button>

    <Button dataCy="confirm-button" on:click={onContinue}>Continue</Button>
  </div>
</div>
