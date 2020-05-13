<script>
  import { Title, Text, Button } from "../DesignSystem/Primitive";
  import { SidebarLayout } from "../DesignSystem/Component";
</script>

<style>
  .container {
    max-width: 71.25rem;
    margin: 0 auto;
    min-width: 640px;
  }
  .sectionHeader {
    margin: 16px 0 24px 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 12px;
  }
  .sectionItem {
    padding: 0 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }
  .action {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-left: 16px;
  }
  .tabs {
    display: flex;
    flex-direction: row;
  }
  .tabs button {
    cursor: pointer;
    padding: 8px 16px;
    border-radius: 4px;
    background-color: var(--color-background);
    color: var(--color-foreground-level-6);
    font-family: var(--typeface-medium);
  }
  .tabs button:focus {
    outline: none;
  }
  .tabs button.active {
    background-color: var(--color-foreground-level-1);
    color: var(--color-secondary);
  }
  .tabs button:hover {
    background-color: var(--color-foreground-level-1);
  }
  .tabs button:active {
    background-color: var(--color-foreground-level-1);
    color: var(--color-foreground-level-5);
  }
</style>

<SidebarLayout dataCy="page">
  <div class="container">
    <Title variant="big">Settings</Title>

    <h2 class="sectionHeader">
      <Title variant="large">Version</Title>
    </h2>
    <div class="sectionItem">
      <div class="info">
        <Text>Version 01.45.02</Text>
      </div>
      <div class="action">
        <Text>There’s a new version of Radicle Upstream</Text>
        <Button style="margin-left: 16px;">Update to Version 01.45.03</Button>
      </div>
    </div>

    <h2 class="sectionHeader">
      <Title variant="large">Appearance</Title>
    </h2>
    <div class="sectionItem">
      <div class="info">
        <Text style="font-family: var(--typeface-medium);">Theme</Text>
      </div>
      <div class="action">
        <div class="tabs">
          <button class="active">Light</button>
          <button>Dark</button>
        </div>
      </div>
    </div>

    <h2 class="sectionHeader">
      <Title variant="large">Registry</Title>
    </h2>
    <div class="sectionItem">
      <div class="info">
        <Text style="font-family: var(--typeface-medium);">Network</Text>
      </div>
      <div class="action">
        <div class="tabs">
          <button class="active">Testnet</button>
          <button>Mainnet</button>
        </div>
      </div>
    </div>

    <h2 class="sectionHeader">
      <Title variant="large">Session management</Title>
    </h2>
    <div class="sectionItem">
      <div class="info">
        <Text style="font-family: var(--typeface-medium);">
          Clear local cache
        </Text>
        <Text>Removes all locally-stored temporary data from your device.</Text>
      </div>
      <div class="action">
        <Button variant="outline">Clear cache</Button>
      </div>
    </div>
    <div class="sectionItem">
      <div class="info">
        <Text style="font-family: var(--typeface-medium);">
          Clears all authentication data
        </Text>
        <Text>
          This is similar to how logout works. You will have to create a new
          identity or restore your existing identity.
        </Text>
      </div>
      <div class="action">
        <Button variant="outline">Clear session</Button>
      </div>
    </div>

    <h2 class="sectionHeader">
      <Title variant="large">Developer tools</Title>
    </h2>
    <div class="sectionItem">
      <div class="info">
        <Text style="font-family: var(--typeface-medium);">
          Unlink all unregistered projects from my profile
        </Text>
        <Text>
          This unlinks your local repositories from Upstream. The local data
          will remain on your computer.
        </Text>
      </div>
      <div class="action">
        <Button variant="destructive">Remove</Button>
      </div>
    </div>

  </div>
</SidebarLayout>
