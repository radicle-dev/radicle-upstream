<script>
  import { Button, Input, Text, Title } from "../../Primitive";
  import Copyable from "../Copyable.svelte";
  import QR from "../QR.svelte";

  $: currentlyActive = "send";
</script>

<style>
  .selector {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }
  .selector:hover button.active:not(:hover) {
    background: none;
  }
  .selector button {
    cursor: pointer;
    padding: 20px 16px;
    margin: 0;
    background-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-5);
    font-family: var(--typeface-bold);
    font-size: 1.5rem;
    border: 1px solid transparent;
  }
  .selector button:first-child {
    border-top-left-radius: 4px;
  }
  .selector button:first-child:not(.active) {
    border-bottom: 1px solid var(--color-foreground-level-2);
    border-right: 1px solid var(--color-foreground-level-2);
  }

  .selector button:last-child {
    border-top-right-radius: 4px;
  }
  .selector button:last-child:not(.active) {
    border-bottom: 1px solid var(--color-foreground-level-2);
    border-left: 1px solid var(--color-foreground-level-2);
  }
  .selector button:focus {
    outline: none;
  }
  .selector button.active {
    background-color: var(--color-background);
    color: var(--color-foreground);
  }
  .selector button:hover {
    color: var(--color-foreground-level-6);
  }
  .selector button:active {
    background-color: var(--color-background);
    color: var(--color-foreground);
  }
  .send {
    padding: 1.5rem;
  }
  .receive {
    display: grid;
    padding: 2rem 1.5rem;
    grid-template-rows: 228px auto auto;
    justify-items: center;
  }
  .submit {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
  }
</style>

<div class="send-receive">
  <div class="selector">
    <button
      class:active={'send' === currentlyActive}
      value="send"
      on:click={() => (currentlyActive = 'send')}>
      Send
    </button>
    <button
      class:active={'receive' === currentlyActive}
      value="receive"
      on:click={() => (currentlyActive = 'receive')}>
      Receive
    </button>
  </div>
  {#if currentlyActive === 'send'}
    <div class="send">
      <Title style="padding-bottom: 0.5rem;">To</Title>
      <Input.Text
        placeholder="Enter address or registered handle"
        style="flex: 1; padding-bottom: 1rem;" />
      <Title style="padding-bottom: 0.5rem;">Amount</Title>
      <Input.Text
        placeholder="Enter the amount"
        style="flex: 1; padding-bottom: 1rem;" />
      <Title style="padding-bottom: 0.5rem;">Note</Title>
      <Input.Text
        placeholder="Optional message"
        style="flex: 1; padding-bottom: 1rem;" />
      <div class="submit">
        <Button disabled={true}>Send transaction</Button>
      </div>
    </div>
  {:else if currentlyActive === 'receive'}
    <div class="receive">
      <QR />
      <Title style="padding-bottom: 0.5rem;">Address</Title>
      <Copyable iconSize="normal">
        <Text>1Ao1drv6fyt5ipWWeN5zExM...</Text>
      </Copyable>
    </div>
  {/if}
</div>
