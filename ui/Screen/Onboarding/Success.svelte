<script>
  import { createEventDispatcher } from "svelte";

  import { PeerId } from "../../DesignSystem/Component";
  import { Button } from "../../DesignSystem/Primitive";

  export let peerId = null;

  const dispatch = createEventDispatcher();

  const onKeydown = event => {
    if (event.key === "Enter") {
      next();
    }
  };

  const next = () => {
    dispatch("close");
  };
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>

<svelte:window on:keydown={onKeydown} />

<div class="container">
  <div class="content">
    <h1 style="text-align: center; margin-bottom: 1.5rem;">All set!</h1>

    <PeerId {peerId} dataCy="peerId" />

    <p
      style="text-align: center; width: 23.13rem; margin: 1.75rem 0 1.75rem 0;
      color: var(--color-foreground-level-6);">
      This is your
      <span class="typo-text-bold">Device ID</span>! It's unique to this device.
      You can find it at any time on your Profile or Settings page. You'll need
      to share it with others to collaborate.
    </p>

    <Button dataCy="go-to-profile-button" on:click={next}>
      Go to my projects
    </Button>
  </div>
</div>
