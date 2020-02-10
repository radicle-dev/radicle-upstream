<script>
  import { pop } from "svelte-spa-router";
  import TxRow from "../../components/TxRow.svelte";
  import { Button, Text, Title, Rad, Avatar } from "../../DesignSystem";
  import RowLayout from "../../layouts/helpers/Row.svelte";

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
    <Title style="margin-bottom: 16px;">From</Title>
    <div>
      <Avatar size="big" variant="user" title="Your personal wallet" />
    </div>
  </div>
</TxRow>

<TxRow disabled={true} style="border-radius: 0">
  <div slot="left">
    <Title>Project Registration</Title>
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
    <Title style="color: var(--color-darkgray);">Transaction Fee</Title>
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
    <Title size="big" style="color: var(--color-pink);">Total</Title>
  </div>

  <div slot="right">
    <Rad amount={24} style="margin-right: 24px" size="big" />
  </div>
</TxRow>

<RowLayout style="margin-top: 48px;">
  <div slot="right">
    <Button disabled={false} on:click={pop} variant="primary">Home</Button>
  </div>
</RowLayout>
