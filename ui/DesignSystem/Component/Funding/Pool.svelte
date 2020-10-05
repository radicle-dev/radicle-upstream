<script lang="ts">
  import { Button, Input } from "../../Primitive";

  export let amount: number;
  export let balance: number;
  export let members: string;
  export let onFillUp: () => Promise<void>;
  export let onDrain: () => Promise<void>;
  export let onSave: () => Promise<void>;

  $: membersList = members
    .split(",")
    .map(e => e.trim())
    .filter(e => e.length > 0);
</script>

<style>
  .row {
    padding: 1.75rem 0px;
    display: flex;
    justify-content: space-between;
  }

  header {
    width: 60%;
  }

  .item {
    display: inline-flex;
    align-items: center;
  }

  .item > * {
    margin: 0 5px;
  }
  .row + .row {
    border-top: solid 1px var(--color-foreground-level-2);
  }
</style>

<div class="pool-container">
  <h3>Pool</h3>
  <ul>
    <!-- Make all options below disabled if the pool is disabled -->
    <li class="row">
      <header>
        <p class="typo-text-bold">Monthly contribution</p>
        <p>
          Set a fixed monthly amount to contribute to your pool. With ${amount} per
          month, pool members get ${amount / membersList.length} a month each. This
          is accessible in real time, so if a user is in the pool for 2 days, they
          can already claim $0.95).
        </p>
      </header>
      <div class="item">
        $
        <Input.Text
          dataCy="monthly-contribution"
          bind:value={amount}
          placeholder="100.00"
          style="max-width: 100px; padding-bottom: 10px;" />
        <div />
      </div>
    </li>
    <li class="row">
      <header>
        <p class="typo-text-bold">Balance</p>
        <p>
          The current balance of your pool. Currently ${amount} per month is required
          to keep your support going, so you don’t need to top up for {Math.floor(balance / amount)}
          months.
        </p>
      </header>
      <div class="item">
        <h3>${balance}</h3>
        <Button
          dataCy="fill-pool-button"
          variant="secondary"
          on:click={onFillUp}>
          Fill up your pool 😉
        </Button>
        <Button dataCy="fill-pool-button" variant="outline" on:click={onDrain}>
          Drain up your pool
        </Button>
      </div>
    </li>
    <li class="row">
      <header>
        <p class="typo-text-bold">Edit your pool</p>
        <p>
          These are the projects, users, and teams in your pool. Remove anyone
          you don’t want to support anymore or add new ones you want to start
          supporting.
        </p>
      </header>
      <div class="item">
        <Input.Textarea
          style="width: 25vw"
          bind:value={members}
          placeholder="Enter members here" />
      </div>
    </li>
  </ul>
  <Button on:click={onSave}>Save</Button>
</div>
