<script>
  import { pop } from "svelte-spa-router";
  import TxRow from "../../components/TxRow.svelte";
  import { Button, Flex, Text } from "../../DesignSystem";
  import Avatar from "../../components/Avatar.svelte";
  import Rad from "../../components/Rad.svelte";

  export let name = null;
  export let timestamp = null;
  export let errorMessage = null;
</script>

<style>
  .status {
    padding: 12px 16px 12px 16px;
    border-top-left-radius: 4px;
    border-top-right-radius: 4px;
  }

  .error {
    background-color: var(--color-red);
  }

  .success {
    background-color: var(--color-green);
  }
</style>

{#if errorMessage}
  <div class="status error">
    <Text>Transaction was unsuccessful: {errorMessage}</Text>
  </div>
{:else}
  <div class="status success">
    <Text>Transaction succeeded on {timestamp}</Text>
  </div>
{/if}
<TxRow
  disabled={true}
  style="background-color: var(--color-almostwhite); border-radius: 0; margin:
  -1px 0 -1px 0; border-top: none">
  <div slot="left">
    <Text variant="title" style="margin-bottom: 16px;">From</Text>
    <div>
      <Avatar size="big" variant="user" title="Your personal wallet" />
    </div>
  </div>
</TxRow>

<TxRow disabled={true} style="border-radius: 0">
  <div slot="left">
    <Text variant="title">Project Registration</Text>
  </div>

  <div slot="right">
    <Avatar
      variant="project"
      imageUrl="https://avatars.dicebear.com/v2/jdenticon/project.svg"
      title={name} />
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

<Flex align="right" style="margin-top: 48px;">
  <Button disabled={false} on:click={pop} variant="primary">Home</Button>
</Flex>
