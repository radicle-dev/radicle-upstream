<script>
  import {
    costSummary,
    formatMessage,
    formatStake,
    formatSubject,
    SubjectType,
    PayerType,
  } from "../../src/transaction.ts";

  import { Avatar, Numeric, Title, Text } from "../../DesignSystem/Primitive";

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
    transactionDeposits
  );

  $: updateAvatar();
</script>

<div dataCy="summary" variant={transaction.id ? 'top' : 'single'}>
  <div
    data-cy="message"
    style="text-align:center; background-color: var(--color-foreground-level-1);
    padding: 48px; border-radius: 4px; border: 1px solid
    var(--color-foreground-level-2); margin-bottom:24px">
    <Title variant="big">{formatMessage(transaction.messages[0])}</Title>
    <caption style="display: inline-flex; margin:16px 0;">
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
    </caption>
  </div>
</div>

<Row dataCy="deposit" variant="top" style="">
  <div slot="left">
    <Text variant="regular" style="color:var(--color-foreground-level-6);">
      {formatStake(transaction.messages[0])}
    </Text>
  </div>

  <div slot="right">
    <Rad rad={summary.depositRad} usd={summary.depositUsd} />
  </div>
</Row>

<Row dataCy="transaction-fee" variant="middle" style="">
  <div slot="left">
    <Text variant="regular" style="color:var(--color-foreground-level-6);">
      Transaction Fee
    </Text>
  </div>

  <div slot="right">
    <Rad rad={summary.feeRad} usd={summary.feeUsd} />
  </div>
</Row>

<Row dataCy="total" variant="bottom" style="margin-bottom: 24px;">
  <div slot="left">
    <Title style="color: var(--color-foreground-level-6);" variant="medium">
      Total
    </Title>
  </div>

  <div slot="right">
    <Rad rad={summary.totalRad} usd={summary.totalUsd} />
  </div>
</Row>

<Row style="margin-bottom: 24px;">
  <div slot="left">
    <Text variant="regular" style="color:var(--color-foreground-level-6);">
      Transaction ID
    </Text>
  </div>
  <div slot="right">
    <!-- TO DO make transaction ID copyable -->
    {#if transaction.id}
      <Numeric
        variant="regular"
        style="color: var(--color-foreground-level-6); max-width: 24ch;
        overflow: hidden; text-overflow: ellipsis;">
        {transaction.id}
      </Numeric>
    {/if}
  </div>
</Row>

<Row style="">
  <div slot="left">
    <Text style="color: var(--color-foreground-level-6);" variant="regular">
      Funding source
    </Text>
  </div>

  <div slot="right">
    <Avatar
      dataCy="payer-avatar"
      title={payer.name}
      imageUrl={payer.imageUrl}
      avatarFallback={payer.avatarFallback}
      variant={payer.type === PayerType.User ? 'circle' : 'square'}
      style="color: var(--color-foreground-level-6);" />
  </div>
</Row>
