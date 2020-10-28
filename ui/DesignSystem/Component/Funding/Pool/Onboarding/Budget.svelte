<script lang="typescript">
  import { Box, Button, Icon, Input } from "../../../../Primitive";

  export let style = "";
  // The current set monthly contribution value.
  export let currentValue: string = "";
  // Whether there is already an ongoing tx setting the monthly contribution.
  export let ongoing = false;
  // The action to run on edit
  export let onEdit: (value: string) => void;

  $: done = currentValue > 0;
</script>

<style>
  h2,
  p,
  .row {
    margin-top: 1rem;
  }

  p {
    color: var(--color-foreground-level-6);
  }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .row * + * {
    margin-left: 10px;
  }
</style>

<Box {style} {done}>
  <h2>Budget</h2>
  <p>
    Set your monthly budget for outgoing support. This amount will flow to your
    receivers in real time.
  </p>

  <p class="typo-text-bold row">
    <Icon.CurrencyDAI style="fill: var(--color-foreground-level-6)" />
    {currentValue} per month
  </p>

  <Button
    disabled={ongoing}
    on:click={onEdit}
    style="margin-top: 1rem"
    variant={'secondary'}>
    {currentValue === '0' ? 'Set' : 'Edit'}
  </Button>
</Box>
