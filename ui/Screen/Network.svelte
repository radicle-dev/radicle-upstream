<script lang="typescript">
  import { connectedPeers, events } from "../src/localPeer";

  import ConnectionStatus from "../DesignSystem/Component/ConnectionStatus.svelte";
  import Copyable from "../DesignSystem/Component/Copyable.svelte";
  import List from "../DesignSystem/Component/List.svelte";
  import Remote from "../DesignSystem/Component/Remote.svelte";
  import SidebarLayout from "../DesignSystem/Component/SidebarLayout.svelte";
</script>

<style>
  .container {
    height: 100vh;
  }

  .header {
    background-color: var(--color-foreground-level-1);
    display: flex;
    height: 8rem;
  }

  .header-content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin: 0 auto;
    max-width: var(--content-max-width);
    padding: 0 var(--content-padding);
    width: 100%;
  }

  .peer-stats {
    display: flex;
    margin-top: 0.5rem;
  }

  .peer-stats-item {
    color: var(--color-foreground-level-6);
    display: flex;
    margin-right: 1rem;
  }

  .layout {
    display: flex;
    height: 100%;
    margin-bottom: 4rem;
    padding: 0 var(--content-padding);
    width: inherit;
  }

  .center-content {
    margin: 0 auto;
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
  }

  .column-left {
    display: flex;
    flex-direction: column;
    padding-right: 0.75rem;
  }

  .column-right {
    display: flex;
    flex-direction: column;
    padding-left: 0.75rem;
    min-width: var(--content-min-width);
    width: 100%;
  }

  .connected-peers {
    width: 30rem;
  }

  .event-log {
    height: 100%;
    overflow-y: scroll;
  }

  h3 {
    margin-bottom: 1.5rem;
  }
</style>

<SidebarLayout>
  <div class="container">
    <div class="header">
      <div class="header-content">
        <h1>Network</h1>
        <div class="peer-stats typo-mono">
          <div class="peer-stats-item">
            <ConnectionStatus />
            <p style="margin-left: 0.5rem;">3 Peers</p>
          </div>
        </div>
      </div>
    </div>
    <div class="layout center-content">
      <div class="column-left">
        <div class="connected-peers">
          <h3>Connected Peers</h3>
          <Remote store={connectedPeers} let:data={peers}>
            <List items={peers} on:select let:item={peer} style="padding: 0;">
              <Copyable
                showIcon={false}
                styleContent={false}
                copyContent="hybh5cb7spafgs7skjg6qkssts3uxht31zskpgs4ypdzrnaq7ye83k@34.91.29.42:12345"
                notificationText="Peer address copied to clipboad">
                <div style="padding: 0.2rem 0.4rem;">
                  <p class="typo-mono-bold">{peer.addr}</p>
                  <p class="typo-text-small-mono">{peer.peerId}</p>
                </div>
              </Copyable>
            </List>
          </Remote>
        </div>
      </div>
      <div class="column-right">
        <h3>Event log</h3>
        <div class="event-log">
          {#each $events as event}
            <div class="event-log-item">{JSON.stringify(event)}</div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</SidebarLayout>
