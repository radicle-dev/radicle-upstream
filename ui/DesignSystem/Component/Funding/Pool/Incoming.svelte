<script lang="typescript">
  import { Dai } from "../../../Component";
  import { Button } from "../../../Primitive";

  export let amount: string = "0";
  export let onCollect: () => void;
  export let ongoingCollect = false;
  export let style = "";
</script>

<style>
  .incoming-container {
    padding: calc(var(--content-padding) / 2);

    border: 1px solid #ebeff3;
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

  .item > * + * {
    margin-left: 7px;
  }
</style>

<div class="incoming-container" {style}>
  <div class="row">
    <div class="text">
      There’s
      <div class="typo-text-bold">
        <Dai color={'var(--color-secondary)'} style="margin-right: 5px">
          {amount}
        </Dai>
      </div>
      waiting on you from supporters.
    </div>

    <Button
      style="margin-left: 12px;"
      disabled={ongoingCollect}
      on:click={onCollect}
      variant={'secondary'}>
      Cash out
    </Button>
  </div>
</div>
