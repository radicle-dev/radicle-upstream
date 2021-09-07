<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import { getDirectoryPath } from "ui/src/ipc";

  import type { ValidationState } from "ui/src/validation";

  import Button from "./Button.svelte";
  import TextInput from "./TextInput.svelte";

  export let placeholder: string | undefined = undefined;
  export let style: string | undefined = undefined;
  export let path = "";
  export let validation: ValidationState | undefined = undefined;

  const dispatch = createEventDispatcher();

  const openFileDialog = async () => {
    path = await getDirectoryPath();
    if (path) {
      dispatch("selected");
    }
  };
</script>

<style>
  .wrapper {
    display: flex;
    align-items: flex-start;
  }
</style>

<div class="wrapper" {style}>
  <TextInput
    {placeholder}
    {validation}
    value={path}
    disabled
    style="margin-right: 0.5rem; flex: 1" />

  <Button dataCy="choose-path-button" on:click={openFileDialog}>Choose</Button>
</div>
