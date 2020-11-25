<script lang="typescript">
  import { Button } from "../../../../Primitive";
  import { Illustration } from "../../../../Component";

  import { Variant as IllustrationVariant } from "../../../../../src/illustration";

  export let onSkip: () => void;
  export let onContinue: () => void;
</script>

<style>
  h1 {
    padding: 0 var(--content-padding);
  }

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
</style>

<Illustration variant={IllustrationVariant.Money} />
<h1>Stream digital money to all your favorites</h1>
<p>
  Streaming sets up an ethereum smart contract that sends money out certain
  speed, defined by you, to a group of people or projects.
</p>
<div class="submit">
  <Button
    variant="transparent"
    dataCy="skip"
    on:click={onSkip}
    style="margin-right: 1rem">
    Skip
  </Button>

  <Button dataCy="confirm-button" on:click={onContinue}>Continue</Button>
</div>
