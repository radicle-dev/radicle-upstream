<script>
  import Urn from "../Urn.svelte";
  import QR from "../QR.svelte";
  import { Flex } from "../../Primitive";

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
</style>

<div class="receive" data-cy="receive">
  <QR key={accountId} size="136" />
  {#if text}
    <p style="padding-top: 1.5rem; color: var(--color-foreground-level-6)">
      {text}
    </p>
  {/if}
  <p class="typo-text-bold" style="padding: 1.5rem 0 0.5rem 0;">Address</p>
  <Flex align="center">
    <Urn urn={accountId} />
  </Flex>
</div>
