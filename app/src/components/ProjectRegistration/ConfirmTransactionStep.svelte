<script>
  import Row from "./Row.svelte";
  import { Button, ButtonRow, Title, Caption, Rad, Avatar } from "../../DesignSystem";

  export let name = null;

  export let onNextStep = null;
  export let onPreviousStep = null;
</script>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Your order
</Caption>
<Row
  style="margin-bottom: 16px; background-color: var(--color-almostwhite)"
  disabled={true}>
  <div slot="left">
    <Title style="color: var(--color-darkgray);">
      Name registration
    </Title>
  </div>

  <div slot="right">
    <Title style="color: var(--color-darkgray)">“{name}”</Title>
  </div>
</Row>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Order cost
</Caption>

<Row
  disabled={true}
  style="border-bottom-left-radius: 0; border-bottom-right-radius: 0">
  <div slot="left">
    <Title style="color: var(--color-darkgray);">
      Project Registration Fee
    </Title>
  </div>

  <div slot="right">
    <Rad amount={20} style="margin-right: 24px" />
  </div>
</Row>

<Row disabled={true} style="border-radius: 0; margin: -1px 0 -1px 0">
  <div slot="left">
    <Title style="color: var(--color-darkgray);">
      Transaction Fee
    </Title>
  </div>

  <div slot="right">
    <Rad amount={4} style="margin-right: 24px" />
  </div>
</Row>

<Row
  disabled={true}
  style="margin-bottom: 16px; border-top-left-radius: 0;
  border-top-right-radius: 0">
  <div slot="left">
    <Title size="big" style="color: var(--color-pink);">Total</Title>
  </div>

  <div slot="right">
    <Rad amount={24} style="margin-right: 24px" size="big" />
  </div>
</Row>

<Caption style="color: var(--color-darkgray); margin-bottom: 16px">
  Paid by
</Caption>

<Row disabled={true}>
  <div slot="left">
    <Avatar title="Your personal wallet" />
  </div>

  <div slot="right">
    <Rad amount={200} style="margin-right: 24px" size="big" />
  </div>
</Row>

<ButtonRow>
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
    <Button disabled={false} on:click={onNextStep} variant="primary">Pay</Button>
  </div>
</ButtonRow>
