<script lang="typescript">
  import type { Tx } from "ui/src/transaction";
  import { Icon } from "ui/DesignSystem/Primitive";
  import { Label } from "ui/DesignSystem/Component";
  import { getDay, getShortMonth } from "ui/src/transaction";

  export let tx: Tx;

  const price = 0.0043;
</script>

<style>
  .container {
    display: flex;
    padding: 0.75rem;
    border-bottom: 1px solid var(--color-foreground-level-2);
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
  }

  .container:last-child {
    border-bottom: 0;
  }

  .left {
    display: flex;
    align-items: center;
  }

  .date {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 2rem;
    margin-right: 1rem;
  }

  .date h5 {
    color: var(--color-foreground-level-4);
    line-height: 1rem;
  }

  .date p {
    color: var(--color-foreground-level-5);
  }
</style>

<div class="container" on:click>
  <div class="left">
    <div class="date">
      <h5>{getShortMonth(new Date(tx.date))}</h5>
      <p class="typo-text-bold">{getDay(new Date(tx.date))}</p>
    </div>
    <Icon.Registered />
    <p class="typo-text-bold" style="margin-left: 0.5rem">{tx.kind}</p>
  </div>
  <Label
    title={`-${price} ETH`}
    color="var(--color-negative-level-1)"
    style="color: var(--color-negative)" />
</div>
