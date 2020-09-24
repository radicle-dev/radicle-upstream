<script lang="ts">
  import { Button, Input } from "../../DesignSystem/Primitive";
  import { SegmentedControl } from "../../DesignSystem/Component";
  import Members from "../Org/Members.svelte";

  // TODO(nuno): receive these instead
  let amount = "100";
  const balance = 390;
  let enabled = false;
  const members = [`juliendonck`, `monadic`, `rudolfs`, `nakamoto`];

  const options = [
    {
      title: "On",
      value: "On",
    },
    {
      title: "Off",
      value: "Off",
    },
  ];

  const onOptInChange = (option: string) => {
    enabled = option == "On";
    console.log("User changed opt in to ", option);
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
  <h3>Pool</h3>
  <ul>
    <li class="row">
      <header>
        <p class="typo-text-bold">
          Contribute monthly funding to projects and users
        </p>
        <p>
          Contribute exactly how much you can afford to open source projects on
          an easy monthly basis.
        </p>
      </header>
      <div class="item">
        <SegmentedControl
          active={enabled ? 'On' : 'Off'}
          {options}
          on:select={option => onOptInChange(option.detail)} />
      </div>
    </li>
    <!-- Make all options below disabled if the pool is disabled -->
    {#if enabled}
      <li class="row">
        <header>
          <p class="typo-text-bold">Monthly contribution</p>
          <p>
            Set a fixed monthly amount to contribute to your pool. With ${amount}
            per month, pool members get ${parseInt(amount) / members.length} a month
            each. This is accessible in real time, so if a user is in the pool for
            2 days, they can already claim $0.95).
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
            to keep your support going, so you don’t need to top up for {Math.floor(balance / parseInt(amount))}
            months.
          </p>
        </header>
        <div class="item">
          <h3>${balance}</h3>
          <Button dataCy="fill-pool-button" variant="secondary">
            Fill up your pool 😉
          </Button>
          <Button dataCy="fill-pool-button" variant="outline">
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
          <!-- TODO(nuno): build based on provided list of members -->
          {members}
        </div>
      </li>
    {/if}
  </ul>
</div>
