<script lang="typescript">
  import * as org from "../src/org";
  import { store } from "../src/wallet";

  import { SidebarLayout } from "../DesignSystem/Component";
  import { Icon } from "../DesignSystem/Primitive";

  export let params: { address: string };

  const safeAddr = org.getSafeAddr(params.address, $store.provider);
</script>

<style>
  header {
    display: flex;
    flex-direction: column;
    justify-content: center;
    max-width: var(--content-max-width);
    background-color: var(--color-foreground-level-1);
    gap: 0.5rem;
    width: 100%;
    margin: 0 auto;
    padding: 0 var(--content-padding);
    height: 12.5rem;
  }
  .safe-addr {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>

<SidebarLayout dataCy="org-screen">
  <header>
    <h1>{params.address}</h1>
    {#await safeAddr}
      Loading..
    {:then addr}
      <div class="safe-addr">
        <Icon.Gnosis />
        {addr}
      </div>
    {:catch error}
      Error:
      {error}
    {/await}
  </header>
  <p>projects</p>
</SidebarLayout>
