<script>
  import { createEventDispatcher } from "svelte";

  import { ShareableIdentifier } from "../../DesignSystem/Component";
  import { Button } from "../../DesignSystem/Primitive";

  export let id = null;

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

    <ShareableIdentifier urn={id} />

    <p
      style="text-align: center; width: 23.13rem; margin: 1.75rem 0 1.75rem 0;
      color: var(--color-foreground-level-6);">
      This is your peerID — it's unique to your machine!<br /> Click to copy it and
      share with others so that they can add you as a remote of a project.
    </p>

    <Button dataCy="go-to-profile-button" on:click={next}>
      Go to my projects
    </Button>
  </div>
</div>
