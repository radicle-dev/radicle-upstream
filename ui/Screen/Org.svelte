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
    width: 100%;
    margin: 0 auto;
    padding: 0 var(--content-padding);
    gap: 0.5rem;
    height: 12.5rem;
  }
  .safe-addr {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  main {
    display: grid;
    grid-template-columns: 1fr 1fr;
    max-width: var(--content-max-width);
    width: 100%;
    margin: 0 auto;
    padding: var(--content-padding);
    gap: 2rem;
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
  <main>
    <div class="projects">
      <h3>Projects</h3>
      <ul>
        <li>project 1</li>
        <li>project 2</li>
      </ul>
    </div>
    <div class="members">
      <h3>Members</h3>
      <ul>
        <li>member 1</li>
        <li>member 2</li>
      </ul>
    </div>
  </main>
</SidebarLayout>
