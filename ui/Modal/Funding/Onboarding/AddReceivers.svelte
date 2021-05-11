<script lang="typescript">
  import { Button, Emoji } from "../../../DesignSystem/Primitive";

  import Receivers from "../../../DesignSystem/Component/Funding/Pool/Receivers.svelte";
  import type { Receivers as PoolReceivers } from "../../../src/funding/pool";

  export let receivers: PoolReceivers;
  export let onBack: () => void;
  export let onContinue: () => void;

  const onKeydown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      onContinue();
    }
  };

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

</style>

<svelte:window on:keydown={onKeydown} />

<Emoji emoji="💸" size="huge" />
<h1>Add receivers</h1>
<p>
  Add receivers to your outgoing support by clicking the “Support” button on
  profiles or project pages.
</p>
<p class="typo-text-small" style="color: var(--color-foreground-level-5)">
  Tip: You can also add new receivers later on by clicking on “Support” on their
  profile.
</p>
<Receivers
  bind:receivers
  editing={true}
  style="margin-top: 1.5rem"
  alignment="center" />
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
