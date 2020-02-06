<script>
  import Row from "./Row.svelte";
  import { Button, ButtonRow, Title, Caption, Rad, Avatar } from "../../DesignSystem";

  export let onNextStep = null;
  export let onPreviousStep = null;
</script>

<style>
  .button-row {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-top: 48px;
  }
</style>

<Row style="margin-bottom: 48px;" active={true}>
  <div slot="left" style="display: flex; align-items: center">
    <Avatar style="margin-right: 16px" />
    <Title style="color: var(--color-darkgray);">
      Your personal wallet
    </Title>
  </div>

  <div slot="right">
    <Rad amount={200} style="margin-right: 24px" />
  </div>
</Row>

<Caption style="color: var(--color-darkgray); margin-bottom: 24px">
  Project wallets
</Caption>
<Row style="margin-bottom: 16px">
  <div slot="left" style="display: flex; align-items: center">
    <Avatar
      imageUrl="https://avatars.dicebear.com/v2/jdenticon/project1.svg"
      style="margin-right: 16px"
      variant="project" />
    <Title style="color: var(--color-darkgray)">
      Monadic wallet
    </Title>
  </div>

  <div slot="right">
    <Rad amount={2387} style="margin-right: 24px" />
  </div>
</Row>

<Row style="margin-bottom: 16px">
  <div slot="left" style="display: flex; align-items: center">
    <Avatar
      imageUrl="https://avatars.dicebear.com/v2/jdenticon/project2.svg"
      style="margin-right: 16px"
      variant="project" />
    <Title style="color: var(--color-darkgray)">
      Radicle wallet
    </Title>
  </div>

  <div slot="right">
    <Rad amount={2387} style="margin-right: 24px" />
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
    <Button disabled={false} on:click={onNextStep} variant="primary">Next</Button>
  </div>
</ButtonRow>
