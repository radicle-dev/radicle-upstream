<script>
  import {
    Avatar,
    Caption,
    Numeric,
    Title
  } from "../../DesignSystem/Primitive";
  import Rad from "./Rad.svelte";
  import Row from "./Transaction/Row.svelte";

  // transaction = {
  //   id: transaction id (optional),
  //   message: "transaction message",
  //   stake: "name of stake (optional)",
  //   subject: {
  //     name: "name of the transaction target",
  //     kind: "project" || "user",
  //     avatarFallback: "avatar fallback of the target",
  //     imageUrl: "avatar url of the target (optional)"
  //   },
  //   payer: {
  //     name: "name of the owner of the paying wallet"
  //     avatar: "avatar of the owner (optional)",
  //     avatarFallback: "avatar fallback of the owner",
  //     imageUrl: "avatar url of the owner (optional)"
  //   }
  // }
  export let transaction = null;

  const feePosition = transaction.stake ? "middle" : "top";
</script>

<Caption style="color: var(--color-foreground-level-6); margin-bottom: 16px">
  Your transaction
</Caption>
<Row dataCy="summary" variant={transaction.id ? 'top' : 'single'}>
  <div slot="left">
    <Title>{transaction.message}</Title>
  </div>

  <div slot="right">
    <Avatar
      title={transaction.subject.name}
      imageUrl={transaction.subject.imageUrl}
      avatarFallback={transaction.subject.avatarFallback}
      variant={transaction.subject.kind}
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

{#if transaction.stake}
  <Row variant="top" style="background-color: var(--color-foreground-level-1)">
    <div slot="left">
      <Title>{transaction.stake}</Title>
    </div>

    <div slot="right">
      <Rad amount={20} />
    </div>
  </Row>
{/if}

<Row
  variant={feePosition}
  style="background-color: var(--color-foreground-level-1)">
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
      title={transaction.payer.name}
      imageUrl={transaction.payer.imageUrl}
      avatarFallback={transaction.payer.avatarFallback}
      style="color: var(--color-foreground)" />
  </div>

  <div slot="right">
    <Rad amount={200} size="big" />
  </div>
</Row>
