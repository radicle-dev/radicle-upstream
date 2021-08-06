<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as modal from "ui/src/modal";
  import { Button } from "ui/DesignSystem";

  export let canCancel: boolean = true;
  export let confirmCopy: string = "Confirm";
  export let cancelCopy: string = "Cancel";
  export let canSubmit: boolean = true;
  export let disableButtons: boolean = false;
  export let onSubmit: () => void = () => {};
  export let onCancel: () => void = () => {
    modal.hide();
  };
</script>

<style>
  .button-row {
    margin-top: 24px;
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
</style>

<div class="button-row">
  {#if canCancel}
    <Button disabled={disableButtons} variant="transparent" on:click={onCancel}
      >{cancelCopy}</Button>
  {/if}
  {#if canSubmit}
    <Button disabled={disableButtons} on:click={onSubmit}>{confirmCopy}</Button>
  {/if}
</div>
