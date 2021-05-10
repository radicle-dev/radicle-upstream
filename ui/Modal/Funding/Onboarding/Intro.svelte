<script lang="typescript">
  import { Button, Emoji } from "../../../DesignSystem/Primitive";

  export let onCancel: () => void;
  export let onContinue: () => void;

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      onContinue();
    }
  };

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

<svelte:window on:keydown={onKeydown} />

<Emoji emoji="💸" size="huge" />
<h1>Stream digital money to all your favorites</h1>
<p>
  Streaming sets up an Ethereum smart contract that sends money out certain
  speed, defined by you, to a group of people or projects.
</p>
<div class="submit">
  <Button
    variant="transparent"
    dataCy="cancel"
    on:click={onCancel}
    style="margin-right: 1rem">
    Cancel
  </Button>

  <Button dataCy="confirm-button" on:click={onContinue}>Continue</Button>
</div>
