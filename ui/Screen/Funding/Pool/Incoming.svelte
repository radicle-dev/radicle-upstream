<script lang="typescript">
  import { Dai } from "../../../DesignSystem/Component";
  import { Button } from "../../../DesignSystem/Primitive";

  export let amount: string = "0";
  export let onCollect: () => void;
  export let ongoingCollect = false;
  export let style = "";
</script>

<style>
  .incoming-container {
    padding: calc(var(--content-padding) / 2);

    border: 1px solid var(--color-foreground-level-2);
    box-sizing: border-box;
    border-radius: 8px;
    background-color: var(--color-secondary-level-1);
  }

  .row {
    display: flex;
    justify-content: space-between;
  }

  .text {
    display: flex;
    align-items: center;
    color: var(--color-secondary);
  }
</style>

<div class="incoming-container" {style}>
  <div class="row">
    <div class="text">
      There’s
      <div class="typo-text-bold">
        <Dai color="var(--color-secondary)" style="margin: 0 7px">{amount}</Dai>
      </div>
      waiting on you from supporters.
    </div>

    <Button
      style="margin-left: 12px;"
      disabled={ongoingCollect}
      on:click={onCollect}
      variant={'secondary'}>
      Collect
    </Button>
  </div>
</div>
