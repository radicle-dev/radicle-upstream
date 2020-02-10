<script>
  import TxRow from "../../components/TxRow.svelte";
  import { Button, Text, Rad, Avatar } from "../../DesignSystem";
  import RowLayout from "../../layouts/helpers/Row.svelte";

  export let name = null;

  export let onNextStep = null;
  export let onPreviousStep = null;
</script>

<Text
  variant="caption"
  style="color: var(--color-darkgray); margin-bottom: 16px">
  Your order
</Text>
<TxRow
  style="margin-bottom: 16px; background-color: var(--color-almostwhite)"
  disabled={true}>
  <div slot="left">
    <Text variant="title" style="color: var(--color-darkgray);">
      Name registration
    </Text>
  </div>

  <div slot="right">
    <Text variant="title" style="color: var(--color-darkgray)">“{name}”</Text>
  </div>
</TxRow>

<Text
  variant="caption"
  style="color: var(--color-darkgray); margin-bottom: 16px">
  Order cost
</Text>

<TxRow
  disabled={true}
  style="border-bottom-left-radius: 0; border-bottom-right-radius: 0">
  <div slot="left">
    <Text variant="title" style="color: var(--color-darkgray);">
      Project Registration Fee
    </Text>
  </div>

  <div slot="right">
    <Rad amount={20} style="margin-right: 24px" />
  </div>
</TxRow>

<TxRow disabled={true} style="border-radius: 0; margin: -1px 0 -1px 0">
  <div slot="left">
    <Text variant="title" style="color: var(--color-darkgray);">
      Transaction Fee
    </Text>
  </div>

  <div slot="right">
    <Rad amount={4} style="margin-right: 24px" />
  </div>
</TxRow>

<TxRow
  disabled={true}
  style="margin-bottom: 16px; border-top-left-radius: 0;
  border-top-right-radius: 0">
  <div slot="left">
    <Text variant="bigTitle" style="color: var(--color-pink);">Total</Text>
  </div>

  <div slot="right">
    <Rad amount={24} style="margin-right: 24px" size="big" />
  </div>
</TxRow>

<Text
  variant="caption"
  style="color: var(--color-darkgray); margin-bottom: 16px">
  Paid by
</Text>

<TxRow disabled={true}>
  <div slot="left">
    <Avatar title="Your personal wallet" />
  </div>

  <div slot="right">
    <Rad amount={200} style="margin-right: 24px" size="big" />
  </div>
</TxRow>

<RowLayout style="margin-top: 48px;">
  <div slot="left">
    <Button
      disabled={false}
      on:click={onPreviousStep}
      variant="outline"
      style="margin-right: 24px">
      Back
    </Button>
  </div>
  <div slot="right">
    <Button disabled={false} on:click={onNextStep} variant="primary">
      Pay
    </Button>
  </div>
</RowLayout>
