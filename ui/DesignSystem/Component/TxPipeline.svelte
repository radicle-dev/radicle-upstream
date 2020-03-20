<script>
  import { Flex, Icon, Text } from "../Primitive";

  const transactions = ["foobar", "foo", "bar"];
  let hidden = true;

  const hideCards = () => {
    hidden = hidden ? false : true;
  };
</script>

<style>
  .pipeline {
    border: 1px solid var(--color-lightgray);
    border-radius: 4px;
    width: 266px;
  }

  .cards {
    background-color: var(--color-almostwhite);
    border-radius: 3px 3px 0 0;
  }

  .card {
    border-bottom: 1px solid var(--color-lightgray);
    height: 56px;
  }

  .summary {
    height: 56px;
  }

  .hidden {
    display: none;
  }
</style>

<div class="pipeline">
  <div class="cards" class:hidden>
    {#each transactions as tx}
      <div class="card">
        <Flex>
          <div slot="left">
            <Text variant="small" style="width: max-content;">{tx}</Text>
          </div>
          <div slot="right">
            <Icon.CarretBig style="vertical-align: middle;" />
          </div>
        </Flex>
      </div>
    {/each}
  </div>
  <div class="summary" on:click={hideCards}>
    <Flex>
      <div slot="left">
        <Text variant="small" style="width: max-content;">
          3 Transactions pending
        </Text>
      </div>
      <div slot="right">
        <Icon.Source
          style="vertical-align: middle; transform: rotate(90deg);" />
      </div>
    </Flex>
  </div>
</div>
