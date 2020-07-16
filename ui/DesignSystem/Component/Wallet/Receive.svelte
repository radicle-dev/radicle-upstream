<script>
  import Urn from "../Urn.svelte";
  import QR from "../QR.svelte";
  import { Text, Title } from "../../Primitive";

  export let address = null;
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

<div class="receive">
  <QR key={address} size="136" color="var(--color-primary)" />
  {#if text}
    <Text style="padding-top: 1.5rem; color: var(--color-foreground-level-6)">
      {text}
    </Text>
  {/if}
  <Title style="padding: 1.5rem 0 0.5rem 0;">Address</Title>
  <Urn urn={address} />
</div>
