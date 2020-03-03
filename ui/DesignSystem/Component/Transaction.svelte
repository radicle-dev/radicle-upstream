<script>
  import { Caption, Title } from "../../DesignSystem/Primitive";
  import Avatar from "./Avatar.svelte";
  import Rad from "./Rad.svelte";
  import TxRow from "./Transaction/Row.svelte";

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
  export let tx = {};

  const feePosition = tx.stake ? "middle" : "top";
</script>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Your transaction
</Caption>
<TxRow style="margin-bottom: 32px;" disabled={true}>
  <div slot="left">
    <Title>{tx.message}</Title>
  </div>

  <div slot="right">
    <Avatar
      title={tx.subject.name}
      imgUrl={tx.subject.avatar}
      variant={tx.subject.kind} />
  </div>
</TxRow>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Transaction cost
</Caption>

{#if tx.stake}
  <TxRow
    disabled={true}
    variant="top"
    style="background-color: var(--color-almostwhite)">
    <div slot="left">
      <Title>{tx.stake}</Title>
    </div>

    <div slot="right">
      <Rad amount={20} style="margin-right: 24px" />
    </div>
  </TxRow>
{/if}

<TxRow
  disabled={true}
  variant={feePosition}
  style="background-color: var(--color-almostwhite)">
  <div slot="left">
    <Title>Transaction Fee</Title>
  </div>

  <div slot="right">
    <Rad amount={4} style="margin-right: 24px" size="big" />
  </div>
</TxRow>

<TxRow
  disabled={true}
  variant="bottom"
  style="margin-bottom: 32px; background-color: var(--color-almostwhite)">
  <div slot="left">
    <Title style="color: var(--color-pink);" variant="big">Total</Title>
  </div>

  <div slot="right">
    <Rad amount={24} style="margin-right: 24px" size="big" />
  </div>
</TxRow>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Paid by
</Caption>

<TxRow disabled={true} style="background-color: var(--color-almostwhite)">
  <div slot="left">
    <Avatar title={tx.payer.name} imgUrl={tx.payer.avatar} />
  </div>

  <div slot="right">
    <Rad amount={200} style="margin-right: 24px" size="big" />
  </div>
</TxRow>
