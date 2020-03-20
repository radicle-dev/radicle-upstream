<script>
  import { Code, Icon, Title } from "../Primitive";

  export let style = null;
  export let peerCount = null;
</script>

<style>
  .toggle {
    display: flex;
    height: 36px;
    align-items: center;
    border-radius: 2px;
    border: 1px solid var(--color-blue);
    background-color: var(--color-white);
    color: var(--color-white);
  }

  .left {
    background-color: var(--color-blue);
    display: flex;
    height: 36px;
    align-items: center;
  }

  .right {
    background-color: var(--color-white);
    color: var(--color-blue);
    height: 34px;
    align-items: center;
    display: flex;
  }
</style>

<div class="toggle" {style}>
  <div class="left">
    <Icon.Peer style="fill: var(--color-white); margin: 0 8px 0 12px" />
    <Title style="margin-right: 8px">Track</Title>
  </div>
  <div class="right">
    <Code variant="medium" style="margin: 0 8px 0 8px">{peerCount}</Code>
  </div>
</div>
