<script lang="ts">
  import { push } from "svelte-spa-router";

  import { Button, Icon, Input } from "../../../Primitive";
  import { Remote, StatefulButton } from "../../../Component";

  import * as path from "../../../../src/path";
  // N.B: Without this alias, rollup runs into issues importing 'Pool' or 'as pool'.
  import * as p from "../../../../src/funding/pool";

  export let pool: p.Pool;

  // TODO(nuno): read from pool
  let monthlyContribution = 0;

  // Necessary to comply with Textarea.bind:value type.
  let members: string | any = "a,b,c"; // TODO(nuno): read from pool: .split(", ")

  const openSendModal = () => {
    p.store.set(pool);
    push(path.poolTopUp());
  };
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
  <h3>Give</h3>

  <Remote store={pool.data} let:data={poolData}>
    <ul>
      <!-- Make all options below disabled if the pool is disabled -->
      <li class="row">
        <header>
          <p class="typo-text-bold">Balance</p>
          <p>
            The current balance of your pool. Currently ${poolData.amountPerBlock}
            per month is required to keep your support going, so you don’t need to
            top up for {Math.floor(poolData.amountPerBlock / poolData.balance)} months.
          </p>
        </header>
        <div class="item">
          <h3>${poolData.balance}</h3>
          <Button
            dataCy="top-up-pool-button"
            variant="secondary"
            on:click={openSendModal}>
            Top up your pool 😉
          </Button>
          <Button
            dataCy="drain-pool-button"
            variant="outline"
            on:click={() => {
              console.log('Open modal to input amount');
            }}>
            Drain up your pool
          </Button>
        </div>
      </li>
      <li class="row">
        <header>
          <p class="typo-text-bold">Monthly contribution</p>
          <p>
            Set a fixed monthly amount to contribute to your pool. With ${poolData.amountPerBlock}
            per month, pool members get ${poolData.amountPerBlock / poolData.receiverAddresses.length}
            a month each. This is accessible in real time, so if a user is in the
            pool for 2 days, they can already claim $0.95).
          </p>
        </header>
        <div class="item">
          <Input.Text
            dataCy="modal-amount-input"
            placeholder="Enter the amount"
            bind:value={monthlyContribution}
            showLeftItem
            autofocus
            style="max-width: 200px;">
            <div slot="left" style="display: flex;">
              <Icon.CurrencyRAD style="fill: var(--color-foreground-level-6)" />
            </div>
          </Input.Text>
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

    <StatefulButton
      title={'Save'}
      onClick={() => pool.save({
          monthlyContribution,
          members: members.split(', '),
        })}
      variant={'primary'}
      successMessage={'✓ Pool successfully saved'}
      errorMessage={e => `Failed to save pool: ${e}`} />
  </Remote>
</div>
