<script>
  import Urn from "../Urn.svelte";
  import QR from "../QR.svelte";
  import { Text, Title } from "../../Primitive";

  export let accountId = null;
  export let text = null;
</script>

<style>
  .receive {
    display: flex;
    flex-direction: column;
    padding: 2rem 1.5rem;
    text-align: center;
  }
  .center {
    display: flex;
    justify-content: center;
  }
</style>

<div class="receive" data-cy="receive">
  <QR key={accountId} size="136" />
  {#if text}
    <Text style="padding-top: 1.5rem; color: var(--color-foreground-level-6)">
      {text}
    </Text>
  {/if}
  <Title style="padding: 1.5rem 0 0.5rem 0;">Address</Title>
  <div class="center">
    <Urn urn={accountId} />
  </div>
</div>
