<script>
  import {
    Avatar,
    Caption,
    Numeric,
    Title
  } from "../../DesignSystem/Primitive";
  import Rad from "./Rad.svelte";
  import Row from "./Transaction/Row.svelte";

  // tx = {
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
  export let tx = null;

  const feePosition = tx.stake ? "middle" : "top";
</script>

<Caption style="color: var(--color-foreground-level-6); margin-bottom: 16px">
  Your transaction
</Caption>
<Row dataCy="tx-summary" variant={tx.id ? 'top' : 'single'}>
  <div slot="left">
    <Title>{tx.message}</Title>
  </div>

  <div slot="right">
    <Avatar
      title={tx.subject.name}
      imageUrl={tx.subject.imageUrl}
      avatarFallback={tx.subject.avatarFallback}
      variant={tx.subject.kind}
      style="color: var(--color-foreground)" />
  </div>
</Row>

{#if tx.id}
  <Row
    variant="bottom"
    style="height: 32px; background-color: var(--color-foreground-level-1)">
    <div slot="left">
      <Numeric variant="tiny" style="color: var(--color-foreground-level-6)">
        {tx.id}
      </Numeric>
    </div>
  </Row>
{/if}

<Caption
  style="color: var(--color-foreground-level-6); margin-bottom: 16px;
  margin-top: 32px;">
  Transaction cost
</Caption>

{#if tx.stake}
  <Row variant="top" style="background-color: var(--color-foreground-level-1)">
    <div slot="left">
      <Title>{tx.stake}</Title>
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
      title={tx.payer.name}
      imageUrl={tx.payer.imageUrl}
      avatarFallback={tx.payer.avatarFallback}
      style="color: var(--color-foreground)" />
  </div>

  <div slot="right">
    <Rad amount={200} size="big" />
  </div>
</Row>
