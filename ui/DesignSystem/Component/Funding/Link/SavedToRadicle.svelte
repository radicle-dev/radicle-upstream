<script lang="typescript">
  import { Button } from "../../../Primitive";
  import { Illustration } from "../../../Component";

  import { Variant as IllustrationVariant } from "../../../../src/illustration";

  export let onCancel: () => void;
  export let onContinue: () => void;
</script>

<style>
  .wrapper {
    display: flex;
    justify-content: space-around;
    align-items: center;
    flex-direction: column;
    min-height: 400px;
  }

  header {
    padding: 0px var(--content-padding);
  }

  .submit {
    display: flex;
    justify-content: flex-end;
    width: 100%;
    margin-top: calc(var(--content-padding) / 2);
  }
</style>

<div class="wrapper">
  <Illustration variant={IllustrationVariant.Socks} />

  <header>
    <h1 style="margin-top: 1.5rem;">
      Your Ethereum address was saved to Radicle
    </h1>
    <p
      style="margin-top: 1.5rem; padding: 0 var(--content-padding)"
      class="typo-text">
      Continue to add your Radicle Identity to Ethereum?
    </p>
  </header>

  <div class="submit">
    <Button variant="transparent" dataCy="cancel-topup" on:click={onCancel}>
      Cancel
    </Button>

    <Button
      dataCy="confirm-button"
      on:click={onContinue}
      style="margin-left: 14px;">
      Continue
    </Button>
  </div>
</div>
