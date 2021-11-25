<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { TextInputValidationState } from "design-system/TextInput";

  import { createEventDispatcher } from "svelte";
  import { getDirectoryPath } from "ui/src/ipc";

  import Button from "design-system/Button.svelte";
  import TextInput from "design-system/TextInput.svelte";

  export let placeholder: string | undefined = undefined;
  export let style: string | undefined = undefined;
  export let path = "";
  export let validationState: TextInputValidationState | undefined = undefined;

  const dispatch = createEventDispatcher();

  async function openFileDialog(): Promise<void> {
    path = await getDirectoryPath();
    if (path) {
      dispatch("selected");
    }
  }
</script>

<style>
  .wrapper {
    display: flex;
    align-items: flex-start;
  }
</style>

<div class="wrapper" {style}>
  <TextInput
    on:click={openFileDialog}
    {placeholder}
    {validationState}
    value={path}
    readonly
    style="margin-right: 0.5rem; flex: 1" />

  <Button dataCy="choose-path-button" on:click={openFileDialog}>Choose</Button>
</div>
