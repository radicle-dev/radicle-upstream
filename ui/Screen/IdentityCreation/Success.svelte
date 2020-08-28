<script>
  import { createEventDispatcher } from "svelte";

  import { store } from "../../src/identity.ts";

  import { Copyable, Remote } from "../../DesignSystem/Component";
  import { Button, Flex } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();
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

<div class="container">
  <div class="content">
    <h1 style="text-align: center; margin-bottom: 24px;">All set!</h1>
    <p
      style="text-align: center; width: 370px; margin-bottom: 27px; color:
      var(--color-foreground-level-6);">
      This is your
      <span class="typo-text-bold">Radicle ID</span>
      . Click to copy it and share it with others so that they can find you.
    </p>
    <Remote {store} let:data={identity}>
      <Copyable style="margin-bottom: 27px;">
        <Flex align="left">
          <p
            class="typo-overflow-ellipsis"
            style="color: var(--color-foreground-level-6); max-width: 350px;">
            {identity.shareableEntityIdentifier}
          </p>
        </Flex>
      </Copyable>
    </Remote>

    <Button dataCy="go-to-profile-button" on:click={() => dispatch('close')}>
      Go to my projects
    </Button>
  </div>
</div>
