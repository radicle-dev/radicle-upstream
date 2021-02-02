<script lang="typescript">
  import * as modal from "../src/modal";
  import * as path from "../src/path";
  import { store as transactions, selectedStore } from "../src/transaction";

  import Center from "../DesignSystem/Component/Transaction/Center.svelte";

  const onSelect = (event: { detail: string }) => {
    const hash = event.detail;
    selectedStore.set(hash);
    modal.hide();
    modal.toggle(path.transaction());
  };
</script>

{#if $transactions.length > 0}
  <Center on:select={onSelect} transactions={$transactions} />
{/if}
