<script>
  import { pop } from "svelte-spa-router";
  import Row from "./Row.svelte";
  import { Button, Text, Title, Rad, Avatar } from "../../DesignSystem";

  export let name = null;
  export let timestamp = null;
  export let errorMessage = null;
</script>

<style>
  .button-row {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    margin-top: 48px;
  }

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
<Row
  disabled={true}
  style="background-color: var(--color-almostwhite); border-radius: 0; margin:
  -1px 0 -1px 0; border-top: none">
  <div slot="left">
    <Title style="margin-bottom: 16px;">From</Title>
    <div style="display: flex; align-items: center">
      <Avatar style="margin-right: 16px" size="big" variant="user" />
      <Title size="big" style="white-space: nowrap">Your personal wallet</Title>
    </div>
  </div>
</Row>

<Row disabled={true} style="border-radius: 0">
  <div slot="left" style="display: flex; align-items: center">
    <Title>Project Registration</Title>
  </div>

  <div slot="right" style="display: flex; align-items: center">
    <Avatar
      style="margin-right: 8px;"
      variant="project"
      imageUrl="https://avatars.dicebear.com/v2/jdenticon/project.svg" />
    <Title>{name}</Title>
  </div>
</Row>

<Row disabled={true} style="border-radius: 0; margin: -1px 0 -1px 0">
  <div slot="left" style="display: flex; align-items: center">
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
  <div slot="left" style="display: flex; align-items: center">
    <Title size="big" style="color: var(--color-pink);">Total</Title>
  </div>

  <div slot="right">
    <Rad amount={24} style="margin-right: 24px" size="big" />
  </div>
</Row>

<div class="button-row">
  <Button disabled={false} on:click={pop} variant="primary">Home</Button>
</div>
