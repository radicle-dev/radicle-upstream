<script>
  import {
    costSummary,
    formatMessage,
    formatStake,
    formatSubject,
    SubjectType,
    PayerType,
  } from "../../src/transaction.ts";

  import {
    Avatar,
    Caption,
    Numeric,
    Title,
  } from "../../DesignSystem/Primitive";

  import Rad from "./Rad.svelte";
  import Row from "./Transaction/Row.svelte";

  export let transaction = null;
  export let payer = null;
  export let transactionDeposits = null;

  let avatar;

  const subject = formatSubject(transaction.messages[0]);
  const subjectAvatarShape = () => {
    switch (subject.type) {
      case SubjectType.User:
      case SubjectType.Member:
      case SubjectType.UserProject:
        return "circle";
      default:
        return "square";
    }
  };

  const updateAvatar = async () => (avatar = await subject.avatarSource);

  const summary = costSummary(
    transaction.messages[0].type,
    parseInt(transaction.fee),
    transactionDeposits,
  );

  $: updateAvatar();
</script>

<Caption style="color: var(--color-foreground-level-6); margin-bottom: 16px">
  Your transaction
</Caption>

<Row dataCy="summary" variant={transaction.id ? 'top' : 'single'}>
  <div slot="left" data-cy="message">
    <Title>{formatMessage(transaction.messages[0])}</Title>
  </div>

  <div slot="right" data-cy="subject">
    {#if avatar}
      <Avatar
        title={subject.name}
        imageUrl={avatar.url}
        avatarFallback={avatar.emoji && avatar}
        variant={subjectAvatarShape()}
        style="color: var(--color-foreground)"
        dataCy="subject-avatar" />
    {:else}
      <Title>{subject.name}</Title>
    {/if}
  </div>
</Row>

{#if transaction.id}
  <Row
    variant="bottom"
    style="height: 32px; background-color: var(--color-foreground-level-1)">
    <div slot="left">
      <Numeric variant="tiny" style="color: var(--color-foreground-level-6)">
        {transaction.id}
      </Numeric>
    </div>
  </Row>
{/if}

<Caption
  style="color: var(--color-foreground-level-6); margin-bottom: 16px;
  margin-top: 32px;">
  Transaction cost
</Caption>

<Row
  dataCy="deposit"
  variant="top"
  style="background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Title>{formatStake(transaction.messages[0])}</Title>
  </div>

  <div slot="right">
    <Rad rad={summary.depositRad} usd={summary.depositUsd} />
  </div>
</Row>

<Row
  dataCy="transaction-fee"
  variant="middle"
  style="background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Title>Transaction Fee</Title>
  </div>

  <div slot="right">
    <Rad rad={summary.feeRad} usd={summary.feeUsd} size="big" />
  </div>
</Row>

<Row
  dataCy="total"
  variant="bottom"
  style="margin-bottom: 32px; background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Title style="color: var(--color-primary);" variant="big">Total</Title>
  </div>

  <div slot="right">
    <Rad rad={summary.totalRad} usd={summary.totalUsd} size="big" />
  </div>
</Row>

<Caption style="color: var(--color-foreground-level-6); margin-bottom: 16px">
  Paid by
</Caption>

<Row style="background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Avatar
      dataCy="payer-avatar"
      title={payer.name}
      imageUrl={payer.imageUrl}
      avatarFallback={payer.avatarFallback}
      variant={payer.type === PayerType.User ? 'circle' : 'square'}
      style="color: var(--color-foreground)" />
  </div>

  <div slot="right">
    <Rad rad={999} usd={999} size="big" />
  </div>
</Row>
