<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";

  import { ValidationStatus } from "ui/src/validation";
  import * as onboarding from "ui/src/onboarding";

  import { Button, Emoji, TextInput } from "ui/DesignSystem";

  const dispatch = createEventDispatcher();

  export let handle = "";

  let nameInput: HTMLInputElement;
  let beginValidation = false;

  const validationStore = onboarding.createHandleValidationStore();

  const validationPasses = () =>
    $validationStore.status === ValidationStatus.Success;
  const validationStarted = () =>
    $validationStore.status !== ValidationStatus.NotStarted;

  $: beginValidation && validationStore.validate(handle);
  $: allowNext = (handle && validationPasses()) || !validationStarted();
  $: {
    if (handle.length > 0) {
      handle = onboarding.formatHandleInput(handle);
    }

    // Start validations only after the user enters the least amount of required
    // characters. This is to avoid showing an empty form with a validation
    // message initially as the field is required and the minimum count of
    // characters is 2.
    if (handle.length > 1) {
      beginValidation = true;
    }
  }

  const next = () => {
    if (!allowNext) {
      nameInput.focus();
      return;
    }

    beginValidation = true;
    validationStore.validate(handle);

    if (!validationPasses()) {
      nameInput.focus();
      return;
    }

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

<svelte:window on:keydown={onKeydown} />

<div class="container" data-cy="enter-name-screen">
  <div>
    <h1>
      Hey
      <Emoji emoji="👋 " size="big" style="display: inline;" />
      what should we call you?
    </h1>
    <p>
      You’ll need a display name to use Radicle. This isn’t unique across the
      network, but it will help others recognize you more easily.
    </p>
    <TextInput
      autofocus
      placeholder="Enter a display name (e.g. coolprogrammer3000)"
      bind:inputElement={nameInput}
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
