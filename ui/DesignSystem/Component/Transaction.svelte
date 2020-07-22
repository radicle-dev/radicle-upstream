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

  import {
    Avatar,
    Icon,
    Numeric,
    Title,
    Text,
  } from "../../DesignSystem/Primitive";

  import Copyable from "../../DesignSystem/Component/Copyable.svelte";

  import Rad from "./Rad.svelte";
  import Header from "./Transaction/Header.svelte";
  import Row from "./Transaction/Row.svelte";

  export let transaction = null;
  export let payer = null;
  export let registrationFee = null;

  let avatar;

  const subject = formatSubject(transaction.messages[0]);

  const updateAvatar = async () => (avatar = await subject.avatarSource);

  const summary = costSummary(
    transaction.messages[0].type,
    parseInt(transaction.fee),
    registrationFee
  );

  const txFeeVariant = summary.registrationFeeRad ? "middle" : "top";

  $: updateAvatar();
</script>

<Header {transaction} {avatar} {subject} />

{#if summary.registrationFeeRad}
  <Row dataCy="registration-fee" variant="top" style="">
    <div slot="left">
      <Text variant="regular" style="color:var(--color-foreground-level-6);">
        {formatStake(transaction.messages[0])}
      </Text>
    </div>

    <div slot="right">
      <Rad
        rad={summary.registrationFeeRad}
        usd={summary.registrationFeeUsd}
        variant="deposit" />
    </div>
  </Row>
{/if}

<Row dataCy="transaction-fee" variant={txFeeVariant} style="">
  <div slot="left">
    <Text variant="regular" style="color:var(--color-foreground-level-6);">
      Transaction Fee
    </Text>
  </div>

  <div slot="right">
    <Rad rad={summary.feeRad} usd={summary.feeUsd} />
  </div>
</Row>

<Row
  dataCy="total"
  variant="bottom"
  style="margin-bottom: 24px; border-top: 1px solid
  var(--color-foreground-level-2); ">
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
  <Row variant="top">
    <div slot="left">
      <Text variant="regular" style="color:var(--color-foreground-level-6);">
        Transaction ID
      </Text>
    </div>
    <div slot="right">
      <Copyable
        style="background:var(--color-foreground-level-2); border-radius:2px;
        display:flex; align-items: center; padding: 4px;">
        <Numeric
          variant="small"
          style="color: var(--color-foreground-level-6); max-width: 24ch;
          white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
          {transaction.id}
        </Numeric>
      </Copyable>
    </div>
  </Row>

  <Row variant="bottom" style="margin-bottom: 24px;">
    <div slot="left">
      <Text variant="regular" style="color:var(--color-foreground-level-6);">
        Status
      </Text>
    </div>
    <div slot="right" style="display: flex; align-items: center;">
      {#if iconState(transaction.state) === 'negative'}
        <Icon.Important
          style="margin-right: 8px; fill: var(--color-negative)" />
      {:else if iconState(transaction.state) === 'positive'}
        <Icon.Check
          variant="filled"
          style="margin-right: 8px; fill: var(--color-positive)" />
      {:else}
        <Icon.TransactionState
          progress={iconProgress(transaction.state)}
          style="margin-right: 8px;"
          variant="small"
          state={iconState(transaction.state)} />
      {/if}
      <Text style="align-self: center; color: var(--color-foreground-level-6);">
        {statusText(transaction.state)}
      </Text>
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
