<script lang="ts">
  import { push } from "svelte-spa-router";

  import { Button, Icon, Input } from "../../../Primitive";
  import { Remote, StatefulButton } from "../../../Component";

  import * as path from "../../../../src/path";
  import * as remote from "../../../../src/remote";
  import * as _pool from "../../../../src/funding/pool";
  import {
    amountStore,
    amountValidationStore,
    membersStore,
    membersValidationStore,
  } from "../../../../src/funding/pool";

  import { ValidationStatus } from "../../../../src/validation";

  export let pool: _pool.Pool;

  // The loaded PoolData, updated at on `pool.data.subscribe`.
  let data: _pool.PoolData | undefined;

  pool.data.subscribe(store => {
    if (store.status === remote.Status.Success) {
      const newData = store.data;
      data = newData;
      monthlyContribution = newData.amountPerBlock;
      members = newData.receiverAddresses.join(",");
    }
  });

  let monthlyContribution = "";
  let validatingAmount = false;
  $: amountStore.set(monthlyContribution ? monthlyContribution.toString() : "");
  $: {
    if ($amountStore && $amountStore.length > 0) validatingAmount = true;
    if (validatingAmount) amountValidationStore.validate($amountStore);
  }

  // Necessary type to comply with Textarea.bind:value type.
  let members: string = "";
  let validatingMembers = false;
  $: membersStore.set(members ? members.toString() : "");
  $: {
    if ($membersStore && $membersStore.length > 0) validatingMembers = true;
    if (validatingMembers) membersValidationStore.validate($membersStore);
  }

  $: validInputs =
    $amountValidationStore &&
    $amountValidationStore.status === ValidationStatus.Success &&
    $membersValidationStore &&
    $membersValidationStore.status === ValidationStatus.Success;

  $: saveEnabled =
    validInputs &&
    data &&
    (extractMembers(members).join(",") !== data.receiverAddresses.join(",") ||
      monthlyContribution.valueOf() !== data.amountPerBlock.valueOf());

  const openSendModal = () => {
    _pool.store.set(pool);
    push(path.poolTopUp());
  };

  // Extract the list of members from the provided raw input string
  function extractMembers(raw: string): string[] {
    return raw
      .split(",")
      .map(e => e.trim())
      .filter(e => e.length > 0);
  }
</script>

<style>
  .pool-give-container {
    margin: 20px 0 60px 0px;
  }
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

<div class="pool-give-container">
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
          <h3>
            <currency>DAI</currency>
            {poolData.balance}
          </h3>
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
            Set a fixed monthly amount to contribute to your pool. With ${monthlyContribution}
            per month, pool members get ${parseInt(monthlyContribution) / members.split(',').length}
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
            validation={$amountValidationStore}
            style="max-width: 200px;">
            <div slot="left" style="position: absolute; top: 9px; left: 10px;">
              <Icon.CurrencyDAI style="fill: var(--color-foreground-level-6)" />
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
            validation={$membersValidationStore}
            style="min-width: 400px;"
            bind:value={members}
            placeholder="Enter members here" />
        </div>
      </li>
    </ul>
    <StatefulButton
      disabled={!saveEnabled}
      title={'Save'}
      onClick={() => pool.save({
          amountPerBlock: monthlyContribution,
          receiverAddresses: members.split(', '),
        })}
      variant={'primary'}
      successMessage={'Pool successfully saved'}
      errorMessage={e => `Failed to save pool: ${e}`} />
  </Remote>
</div>
