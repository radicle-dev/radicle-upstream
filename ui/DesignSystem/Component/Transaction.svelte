<script>
  import { Avatar, Caption, Title } from "../../DesignSystem/Primitive";
  import Rad from "./Rad.svelte";
  import Row from "./Transaction/Row.svelte";

  // tx = {
  //   message: "transaction message",
  //   stake: "name of stake (optional)",
  //   subject: {
  //     name: "name of the transaction target",
  //     kind: "project" || "user"
  //     avatar: "avatar url of the target (optional)"
  //   },
  //   payer: {
  //     name: "name of the owner of the paying wallet"
  //     avatar: "avatar of the owner (optional)"
  //   }
  // }
  export let tx = null;

  const feePosition = tx.stake ? "middle" : "top";
</script>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Your transaction
</Caption>
<Row style="margin-bottom: 32px;">
  <div slot="left">
    <Title>{tx.message}</Title>
  </div>

  <div slot="right">
    <Avatar
      title={tx.subject.name}
      imageUrl={tx.subject.avatar}
      variant={tx.subject.kind}
      style="color: var(--color-black)" />
  </div>
</Row>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Transaction cost
</Caption>

{#if tx.stake}
  <Row variant="top" style="background-color: var(--color-almostwhite)">
    <div slot="left">
      <Title>{tx.stake}</Title>
    </div>

    <div slot="right">
      <Rad amount={20} />
    </div>
  </Row>
{/if}

<Row variant={feePosition} style="background-color: var(--color-almostwhite)">
  <div slot="left">
    <Title>Transaction Fee</Title>
  </div>

  <div slot="right">
    <Rad amount={4} size="big" />
  </div>
</Row>

<Row
  variant="bottom"
  style="margin-bottom: 32px; background-color: var(--color-almostwhite)">
  <div slot="left">
    <Title style="color: var(--color-pink);" variant="big">Total</Title>
  </div>

  <div slot="right">
    <Rad amount={24} size="big" />
  </div>
</Row>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Paid by
</Caption>

<Row style="background-color: var(--color-almostwhite)">
  <div slot="left">
    <Avatar
      title={tx.payer.name}
      imageUrl={tx.payer.avatar}
      style="color: var(--color-black)" />
  </div>

  <div slot="right">
    <Rad amount={200} size="big" />
  </div>
</Row>
