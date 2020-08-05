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

  import { Avatar, Icon, Title, Text } from "../../DesignSystem/Primitive";

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

<Header {transaction} {avatar} {subject} accountId={viewerAccountId} />

<Row dataCy="transaction-fee" variant="top" style="">
  <div slot="left">
    <Text variant="regular" style="color:var(--color-foreground-level-6);">
      Transaction fee
    </Text>
  </div>

  <div slot="right">
    <Rad rad={summary.txFee.rad} usd={summary.txFee.usd} />
  </div>
</Row>

{#if summary.registrationFee}
  <Row dataCy="registration-fee" variant="middle" style="">
    <div slot="left">
      <Text variant="regular" style="color:var(--color-foreground-level-6);">
        {formatStake(transaction.messages[0].type)}
      </Text>
    </div>

    <div slot="right">
      <Rad
        rad={summary.registrationFee.rad}
        usd={summary.registrationFee.usd} />
    </div>
  </Row>
{/if}

{#if summary.transferAmount}
  <Row dataCy="transfer-amount" variant="middle" style="">
    <div slot="left">
      <Text variant="regular" style="color:var(--color-foreground-level-6);">
        Amount
      </Text>
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
    <Title style="color: var(--color-foreground-level-6);" variant="medium">
      Total
    </Title>
  </div>

  <div slot="right">
    <Rad rad={summary.total.rad} usd={summary.total.usd} />
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
      <Urn urn={transaction.id} />
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
      dataCy="funding-source"
      title={payer.name}
      imageUrl={payer.imageUrl}
      avatarFallback={payer.avatarFallback}
      variant={payer.type === PayerType.User ? 'circle' : 'square'}
      style="color: var(--color-foreground-level-6);" />
  </div>
</Row>
