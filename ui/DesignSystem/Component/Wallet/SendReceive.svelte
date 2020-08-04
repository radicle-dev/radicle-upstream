<script>
  import { push } from "svelte-spa-router";
  import * as path from "../../../src/path.ts";
  import {
    payerStore,
    recipientStore,
    amountStore,
  } from "../../../src/transfer.ts";

  import { Button, Input, Icon, Title } from "../../Primitive";
  import Receive from "./Receive.svelte";

  export let accountId = null;
  export let id = null;
  let recipient,
    amount,
    currentlyActiveSend = true;

  const openSendModal = () => {
    payerStore.set(id);
    recipientStore.set(recipient);
    amountStore.set(amount);
    push(path.sendFunds());
  };
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
    font-size: 1rem;
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

  .send {
    padding: 1.5rem;
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
      class="semi-bold"
      class:active={currentlyActiveSend}
      value="send"
      data-cy="send-tab"
      on:click={() => (currentlyActiveSend = true)}>
      Send
    </button>
    <button
      class="semi-bold"
      class:active={!currentlyActiveSend}
      value="receive"
      data-cy="receive-tab"
      on:click={() => (currentlyActiveSend = false)}>
      Receive
    </button>
  </div>
  {#if currentlyActiveSend}
    <div class="send" data-cy="send">
      <Title style="padding-bottom: 0.5rem;">To</Title>
      <Input.Text
        dataCy="recipient-input"
        bind:value={recipient}
        placeholder="Enter an account address"
        style="flex: 1; padding-bottom: 1rem;" />
      <Title style="padding-bottom: 0.5rem;">Amount</Title>
      <Input.Text
        dataCy="amount-input"
        bind:value={amount}
        placeholder="Enter the amount"
        showLeftItem
        style="flex: 1; padding-bottom: 1rem;">
        <div slot="left" style="display: flex;">
          <Icon.Currency
            size="big"
            style="fill: var(--color-foreground-level-6)" />
        </div>
      </Input.Text>
      <!-- TODO: Add note back in when implemented on registry
      <Title style="padding-bottom: 0.5rem;">Note</Title>
      <Input.Text
        placeholder="Optional message"
        style="flex: 1; padding-bottom: 1rem;" /> -->
      <div class="submit">
        <Button
          dataCy="send-transaction-button"
          disabled={!recipient || !amount || recipient === '' || amount === ''}
          on:click={openSendModal}>
          Send transaction
        </Button>
      </div>
    </div>
  {:else}
    <Receive {accountId} />
  {/if}
</div>
