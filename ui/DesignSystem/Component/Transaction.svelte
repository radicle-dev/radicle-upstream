<script>
  import {
    formatMessage,
    formatStake,
    Variant
  } from "../../src/transaction.ts";

  import {
    Avatar,
    Caption,
    Numeric,
    Title
  } from "../../DesignSystem/Primitive";

  import Rad from "./Rad.svelte";
  import Row from "./Transaction/Row.svelte";

  export let transaction = null;
  export let payer = null;
  export let subject = null;
</script>

<Caption style="color: var(--color-foreground-level-6); margin-bottom: 16px">
  Your transaction
</Caption>
<Row dataCy="summary" variant={transaction.id ? 'top' : 'single'}>
  <div slot="left">
    <Title>{formatMessage(transaction.messages[0])}</Title>
  </div>

  <div slot="right">
    <Avatar
      title={subject.name}
      imageUrl={subject.imageUrl}
      avatarFallback={subject.avatarFallback}
      variant={subject.variant === Variant.User ? 'circle' : 'square'}
      style="color: var(--color-foreground)" />
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

<Row variant="top" style="background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Title>{formatStake(transaction.messages[0])}</Title>
  </div>

  <div slot="right">
    <Rad amount={20} />
  </div>
</Row>

<Row variant="middle" style="background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Title>Transaction Fee</Title>
  </div>

  <div slot="right">
    <Rad amount={4} size="big" />
  </div>
</Row>

<Row
  variant="bottom"
  style="margin-bottom: 32px; background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Title style="color: var(--color-primary);" variant="big">Total</Title>
  </div>

  <div slot="right">
    <Rad amount={24} size="big" />
  </div>
</Row>

<Caption style="color: var(--color-foreground-level-6); margin-bottom: 16px">
  Paid by
</Caption>

<Row style="background-color: var(--color-foreground-level-1)">
  <div slot="left">
    <Avatar
      title={payer.name}
      imageUrl={payer.imageUrl}
      avatarFallback={payer.avatarFallback}
      variant={payer.variant === Variant.User ? 'circle' : 'square'}
      style="color: var(--color-foreground)" />
  </div>

  <div slot="right">
    <Rad amount={200} size="big" />
  </div>
</Row>
