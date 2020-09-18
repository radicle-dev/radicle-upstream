<script>
  import { createEventDispatcher } from "svelte";

  import { ValidationStatus } from "../../src/validation.ts";
  import * as onboarding from "../../src/onboarding.ts";

  import { Button, Emoji, Input } from "../../DesignSystem/Primitive";

  const dispatch = createEventDispatcher();

  export let handle = null;

  let beginValidation = false;
  const validationStore = onboarding.createHandleValidationStore();

  $: beginValidation && validationStore.validate(handle);
  $: allowNext = (handle && validationPasses()) || !validationStarted();

  const validationPasses = () => {
    return $validationStore.status === ValidationStatus.Success;
  };

  const validationStarted = () => {
    return $validationStore.status !== ValidationStatus.NotStarted;
  };

  const next = () => {
    if (!allowNext) return;

    beginValidation = true;
    validationStore.validate(handle);

    if (!validationPasses()) return;

    dispatch("next", handle);
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
      autofocus={true}
      placeholder="Enter a name"
      bind:value={handle}
      on:enter={next}
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
