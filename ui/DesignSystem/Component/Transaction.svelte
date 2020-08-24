<script>
  import {
    costSummary,
    formatStake,
    formatSubject,
    PayerType,
    iconState,
    iconProgress,
    statusText,
  } from "../../src/transaction.ts";

  import { Avatar, Icon } from "../../DesignSystem/Primitive";
  import TransactionSpinner from "./Transaction/Spinner.svelte";

  import Urn from "../../DesignSystem/Component/Urn.svelte";

  import Rad from "./Rad.svelte";
  import Header from "./Transaction/Header.svelte";
  import Row from "./Transaction/Row.svelte";

  export let transaction = null;
  export let payer = null;
  export let viewerAccountId = null;

  let avatar;

  const subject = formatSubject(transaction.messages[0], viewerAccountId);

  const updateAvatar = async () => (avatar = await subject.avatarSource);

  const summary = costSummary(transaction);

  $: updateAvatar();
</script>

<style>
  .row-text {
    color: var(--color-foreground-level-6);
  }
</style>

<Header {transaction} {avatar} {subject} accountId={viewerAccountId} />

<Row dataCy="transaction-fee" variant="top">
  <div slot="left">
    <p class="row-text">Transaction fee</p>
  </div>

  <div slot="right">
    <Rad rad={summary.txFee.rad} usd={summary.txFee.usd} />
  </div>
</Row>

{#if summary.registrationFee}
  <Row dataCy="registration-fee" variant="middle">
    <div slot="left">
      <p class="row-text">{formatStake(transaction.messages[0].type)}</p>
    </div>

    <div slot="right">
      <Rad
        rad={summary.registrationFee.rad}
        usd={summary.registrationFee.usd} />
    </div>
  </Row>
{/if}

{#if summary.transferAmount}
  <Row dataCy="transfer-amount" variant="middle">
    <div slot="left">
      <p class="row-text">Amount</p>
    </div>

    <div slot="right">
      <Rad rad={summary.transferAmount.rad} usd={summary.transferAmount.usd} />
    </div>
  </Row>
{/if}

<Row
  dataCy="total"
  variant="bottom"
  style="margin-bottom: 24px; border-top: 1px solid
  var(--color-foreground-level-2); ">
  <div slot="left">
    <p class="typo-text-bold">Total</p>
  </div>

  <div slot="right">
    <Rad rad={summary.total.rad} usd={summary.total.usd} />
  </div>
</Row>

{#if transaction.id}
  <Row variant="top">
    <div slot="left">
      <p class="row-text">Transaction ID</p>
    </div>
    <div slot="right">
      <Urn
        urn={transaction.id}
        notificationText="The transaction ID is copied to your clipboard" />
    </div>
  </Row>

  <Row variant="bottom" style="margin-bottom: 24px;">
    <div slot="left">
      <p class="row-text">Status</p>
    </div>
    <div slot="right" style="display: flex; align-items: center;">
      {#if iconState(transaction.state) === 'negative'}
        <Icon.Important
          style="margin-right: 8px; fill: var(--color-negative)" />
      {:else if iconState(transaction.state) === 'positive'}
        <Icon.CheckCircle
          style="margin-right: 8px; fill: var(--color-positive)" />
      {:else}
        <TransactionSpinner
          progress={iconProgress(transaction.state)}
          style="margin-right: 8px;"
          variant="small"
          state={iconState(transaction.state)} />
      {/if}
      <p style="align-self: center;">{statusText(transaction.state)}</p>
    </div>
  </Row>
{/if}

<Row>
  <div slot="left">
    <p class="row-text">Funding source</p>
  </div>

  <div slot="right">
    <Avatar
      dataCy="funding-source"
      title={payer.name}
      imageUrl={payer.imageUrl}
      avatarFallback={payer.avatarFallback}
      variant={payer.type === PayerType.User ? 'circle' : 'square'}
      style="color: var(--color-foreground-level-6);" />
  </div>
</Row>
