<script>
  import { Caption, Title } from "../../DesignSystem/Primitive";
  import Avatar from "./Avatar.svelte";
  import Rad from "./Rad.svelte";
  import TxRow from "./TxRow.svelte";

  export let txMessage = null;
  export let subjectName = null;
  export let subjectAvatar = null;
  export let stakeName = null;
  export let payerName = null;
  export let payerAvatar = null;

  const feePosition = stakeName ? "middle" : "top";
</script>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Your transaction
</Caption>
<TxRow style="margin-bottom: 16px;" disabled={true}>
  <div slot="left">
    <Title>{txMessage}</Title>
  </div>

  <div slot="right">
    <Avatar title={subjectName} imgUrl={subjectAvatar} />
  </div>
</TxRow>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Transaction cost
</Caption>

{#if stakeName}
  <TxRow
    disabled={true}
    variant="top"
    style="background-color: var(--color-almostwhite)">
    <div slot="left">
      <Title>{stakeName}</Title>
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
  style="margin-bottom: 16px; background-color: var(--color-almostwhite)">
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
    <Avatar title={payerName} imgUrl={payerAvatar} />
  </div>

  <div slot="right">
    <Rad amount={200} style="margin-right: 24px" size="big" />
  </div>
</TxRow>
