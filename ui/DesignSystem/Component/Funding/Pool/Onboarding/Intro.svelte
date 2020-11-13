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

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: calc(var(--content-padding) / 2);
  }
</style>

<Illustration variant={IllustrationVariant.Money} />
<h1>Stream digital money to your favorite people</h1>
<p>
  Supporting others let’s you set up an <a href="#TODO">ethereum smart contract</a>
  where you can top up a certain amount to be trickled out at a user defined rate
  to a group of people of your choosing.
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
