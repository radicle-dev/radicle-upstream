<script>
  import { Title, Text, Button } from "../DesignSystem/Primitive";
  import { SidebarLayout, SegmentedControl } from "../DesignSystem/Component";

  const themeOptions = [
    {
      title: "Light",
      value: 0,
    },
    {
      title: "Dark",
      value: 1,
    },
  ];
  const networkOptions = [
    {
      title: "Emulator",
      value: 0,
    },
    {
      title: "FFnet",
      value: 1,
    },
    {
      title: "Testnet",
      value: 2,
    },
  ];
</script>

<style>
  .container {
    max-width: 71.25rem;
    margin: 0 auto;
    min-width: 640px;
  }
  section header {
    margin: 16px 0 24px 0;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 12px;
  }
  .section-item {
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
</style>

<SidebarLayout dataCy="page">
  <div class="container">
    <Title variant="big">Settings</Title>

    <section>
      <header>
        <Title variant="large">Version</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text>Version 01.45.02</Text>
        </div>
        <div class="action">
          <Text>There’s a new version of Radicle Upstream</Text>
          <Button style="margin-left: 16px;">Update to Version 01.45.03</Button>
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Appearance</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Theme</Text>
        </div>
        <div class="action">
          <SegmentedControl
            active={0}
            options={themeOptions}
            on:select={() => console.log('event(select)')} />
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Registry</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Network</Text>
        </div>
        <div class="action">
          <SegmentedControl
            active={0}
            options={networkOptions}
            on:select={() => console.log('event(select)')} />
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Session management</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Clear local cache</Text>
          <Text>
            Removes all locally-stored temporary data from your device.
          </Text>
        </div>
        <div class="action">
          <Button variant="outline">Clear cache</Button>
        </div>
      </div>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">Clears all authentication data</Text>
          <Text>
            This is similar to how logout works. You will have to create a new
            identity or restore your existing identity.
          </Text>
        </div>
        <div class="action">
          <Button variant="outline">Clear session</Button>
        </div>
      </div>
    </section>

    <section>
      <header>
        <Title variant="large">Developer tools</Title>
      </header>
      <div class="section-item">
        <div class="info">
          <Text variant="medium">
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
    </section>

  </div>
</SidebarLayout>
