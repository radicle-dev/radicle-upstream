<script>
  import {
    costSummary,
    formatStake,
    formatSubject,
    PayerType,
  } from "../../src/transaction.ts";

  import {
    Avatar,
    Icon,
    Numeric,
    Title,
    Text,
  } from "../../DesignSystem/Primitive";

  import Rad from "./Rad.svelte";
  import Header from "./Transaction/Header.svelte";
  import Row from "./Transaction/Row.svelte";

  export let transaction = null;
  export let payer = null;
  export let transactionDeposits = null;

  let avatar;

  const subject = formatSubject(transaction.messages[0]);

  const updateAvatar = async () => (avatar = await subject.avatarSource);

  const summary = costSummary(
    transaction.messages[0].type,
    parseInt(transaction.fee),
    transactionDeposits
  );

  $: updateAvatar();
</script>

<Header {transaction} {avatar} {subject} />

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

{#if transaction.id}
  <Row style="margin-bottom: 24px;">
    <div slot="left">
      <Text variant="regular" style="color:var(--color-foreground-level-6);">
        Transaction ID
      </Text>
    </div>
    <div slot="right">
      <!-- TO DO make transaction ID copyable -->
      <div
        style="background:var(--color-foreground-level-2); border-radius:2px;
        display:inline-flex">
        <Numeric
          variant="small"
          style="color: var(--color-foreground-level-6); max-width: 24ch;
          overflow: hidden; text-overflow: ellipsis; padding:3px;">
          {transaction.id}
        </Numeric>
        <Icon.Copy />
      </div>
    </div>
  </Row>
{/if}

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
