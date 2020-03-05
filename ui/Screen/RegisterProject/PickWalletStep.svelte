<script>
  import { Avatar, Button, Flex, Text } from "../../DesignSystem/Primitive";
  import { Rad, Row } from "../../DesignSystem/Component";

  export let onNextStep = null;
  export let onPreviousStep = null;
</script>

<Row style="margin-bottom: 48px;" active={true} disabled={false}>
  <div slot="left">
    <Avatar title="Your personal wallet" />
  </div>

  <div slot="right">
    <Rad amount={200} style="margin-right: 24px" />
  </div>
</Row>

<Text
  variant="caption"
  style="color: var(--color-darkgray); margin-bottom: 24px">
  Project wallets
</Text>
<Row style="margin-bottom: 16px" disabled={false}>
  <div slot="left">
    <Avatar
      imageUrl="https://avatars.dicebear.com/v2/jdenticon/project1.svg"
      variant="project"
      title="Monadic wallet" />
  </div>

  <div slot="right">
    <Rad amount={2387} style="margin-right: 24px" />
  </div>
</Row>

<Row style="margin-bottom: 16px" disabled={false}>
  <div slot="left">
    <Avatar
      imageUrl="https://avatars.dicebear.com/v2/jdenticon/project2.svg"
      variant="project"
      title="Radicle wallet" />
  </div>

  <div slot="right">
    <Rad amount={2387} style="margin-right: 24px" />
  </div>
</Row>

<Flex style="margin-top: 48px;">
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
      Next
    </Button>
  </div>
</Flex>
