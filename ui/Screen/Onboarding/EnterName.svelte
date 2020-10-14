<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { ValidationStatus } from "../../src/validation";
  import * as onboarding from "../../src/onboarding";

  import { Button, Emoji, Input } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();

  export let handle = "";

  $: if (handle.length > 0) handle = onboarding.formatHandleInput(handle);

  let beginValidation = false;
  const validationStore = onboarding.createHandleValidationStore();

  $: beginValidation && validationStore.validate(handle);
  $: allowNext = (handle && validationPasses()) || !validationStarted();

  const validationPasses = () =>
    $validationStore.status === ValidationStatus.Success;

  const validationStarted = () =>
    $validationStore.status !== ValidationStatus.NotStarted;

  const next = () => {
    if (!allowNext) return;

    beginValidation = true;
    validationStore.validate(handle);

    if (!validationPasses()) return;

    dispatch("next", handle);
  };

  const onKeydown = (event: KeyboardEvent) => {
    switch (event.code) {
      case "Enter":
        next();
        break;
    }
  };
</script>

<style>
  .container {
    display: flex;
    align-items: center;
    height: 100%;
  }

  h1 {
    text-align: center;
    width: 22rem;
    margin: 0 auto;
  }

  p {
    margin: 1.25rem 0;
    color: var(--color-foreground-level-6);
  }

  .buttons {
    display: flex;
    justify-content: flex-end;
  }
</style>

<div class="container" data-cy="enter-name-screen">
  <div>
    <h1>
      Hey
      <Emoji emoji="👋 " size="big" style="display: inline;" />
      what should we call you?
    </h1>
    <p>
      You’ll need a display name to interact on Radicle. This isn’t unique
      across the network, but it helps others recognize you a little easier.
    </p>
    <Input.Text
      autofocus
      placeholder="Enter a display name (e.g. coolprogrammer3000)"
      bind:value={handle}
      on:keydown={onKeydown}
      dataCy="handle-input"
      validation={$validationStore}
      hint="↵"
      style="margin: 1rem 0 2rem 0;" />
    <div class="buttons">
      <Button dataCy="next-button" disabled={!allowNext} on:click={next}>
        Looks good
      </Button>
    </div>
  </div>
</div>
