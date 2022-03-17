<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as svelteStore from "svelte/store";

  import { updateChecker } from "ui/src/updateChecker";
  import * as ipc from "ui/src/ipc";
  import * as modal from "ui/src/modal";
  import * as Session from "ui/src/session";

  import Button from "design-system/Button.svelte";
  import SegmentedControl from "design-system/SegmentedControl.svelte";

  import CodeFontSetting from "ui/App/SharedComponents/CodeFontSetting.svelte";
  import CopyableIdentifier from "ui/App/SharedComponents/CopyableIdentifier.svelte";
  import PrimaryColorSetting from "ui/App/SharedComponents/PrimaryColorSetting.svelte";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";
  import ShortcutsModal from "ui/App/ShortcutsModal.svelte";
  import ThemeSetting from "ui/App/SharedComponents/ThemeSetting.svelte";
  import UiFontSetting from "ui/App/SharedComponents/UiFontSetting.svelte";

  let version = "";
  (async () => {
    version = await ipc.getVersion();
  })();

  // We trick TypeScript because svelte cannot deal with type refinement
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const latestVersionInfo: svelteStore.Readable<any> =
    updateChecker.newVersion();

  // This value is not reactive on purpose. We only want to move this to
  // the top on the initial render. Subsequent changes should not mess
  // with the layout as to not confuse the user.
  const showVersionAtTop = Boolean($latestVersionInfo);

  const appUpdateNotificationEnabled = svelteStore.derived(
    updateChecker.isEnabled(),
    isEnabled => (isEnabled ? "on" : "off")
  );

  function setAppUpdateNotificationEnabled(event: CustomEvent): void {
    if (event.detail === "on") {
      updateChecker.enable();
    } else {
      updateChecker.disable();
    }
  }

  const appUpdateNotificationEnabledOptions = [
    { value: "on", title: "Notify Me" },
    { value: "off", title: "Turn off" },
  ];

  const session = Session.unsealed();
</script>

<style>
  .sections {
    display: flex;
    flex-direction: column;
  }

  section header {
    margin: 1rem 0 0.5rem;
    border-bottom: 1px solid var(--color-foreground-level-3);
    padding: 0.75rem;
    display: flex;
    justify-content: space-between;
  }

  .section-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 0.75rem;
  }

  .action {
    display: flex;
    justify-content: flex-end;
    margin-left: 1rem;
    align-items: center;
  }

  .title {
    display: flex;
    justify-content: space-between;
    margin: 2rem 0;
    align-items: flex-end;
    padding: 0 0.75rem;
  }

  .border {
    border-bottom: 1px solid var(--color-foreground-level-1);
  }
</style>

<ScreenLayout dataCy="settings-page">
  <div class="title">
    <h1>Settings</h1>
    <span
      class="typo-link"
      on:click|stopPropagation={() => modal.toggle(ShortcutsModal)}>
      Keyboard shortcuts
    </span>
  </div>
  <div class="sections">
    <section>
      <header>
        <h3>Devices</h3>
      </header>
      <div class="section-item">
        <p>
          Share your Device ID with others to be added as a remote.
          <br />
          <a
            class="typo-link"
            href="https://docs.radicle.xyz/docs/understanding-radicle/faq#can-i-use-radicle-with-multiple-devices"
            >Learn more about managing devices</a>
        </p>
        <div class="action">
          <CopyableIdentifier value={session.identity.peerId} kind="deviceId" />
        </div>
      </div>
    </section>

    <section>
      <header>
        <h3>Appearance</h3>
      </header>
      <div class="section-item border">
        <div>
          <p class="typo-text-bold">Theme</p>
          <p style="color: var(--color-foreground-level-6);">
            Dark mode might be easier on the eyes, but some just want to follow
            the light. Only true h4x0rs know.
          </p>
        </div>
        <div class="action">
          <ThemeSetting />
        </div>
      </div>
      <div class="section-item border">
        <div>
          <p class="typo-text-bold">UI Font</p>
          <p style="color: var(--color-foreground-level-6);">
            This is the font you’ll read most often around the app. In fact,
            it’s the one you’re reading right now.
          </p>
        </div>
        <div class="action">
          <UiFontSetting />
        </div>
      </div>
      <div class="section-item">
        <div>
          <p class="typo-text-bold">Code Font</p>
          <p style="color: var(--color-foreground-level-6);">
            This is the font source code is displayed in on repositories. It’s
            also used for hashes.
          </p>
        </div>
        <div class="action">
          <CodeFontSetting />
        </div>
      </div>
      <div class="section-item">
        <div>
          <p class="typo-text-bold">Color</p>
          <p style="color: var(--color-foreground-level-6);">
            This is the primary color you'll see through the app.
          </p>
        </div>
        <div class="action">
          <PrimaryColorSetting />
        </div>
      </div>
    </section>

    <section>
      <header>
        <h3>Feedback</h3>
      </header>
      <div class="section-item">
        <p class="typo-text-bold">View community forum</p>
        <div class="action">
          <a
            class="typo-link"
            href="https://radicle.community/c/site-feedback/2"
            >radicle.community</a>
        </div>
      </div>
      <div class="section-item">
        <p class="typo-text-bold">Get in touch directly</p>
        <div class="action">
          <a
            class="typo-link"
            href="https://discord.com/channels/841318878125490186/841318878650302488"
            >Radicle Community Discord</a>
        </div>
      </div>
    </section>

    <section data-cy="version" style={showVersionAtTop ? "order: -1" : ""}>
      <header>
        <h3>Version</h3>
      </header>
      <div class="section-item" style="padding-bottom: 0">
        <p style="color: var(--color-foreground-level-6);">
          Version
          {version}
        </p>
        {#if $latestVersionInfo}
          <div class="action">
            There’s a new version of Radicle Upstream
            <Button
              style="margin-left: 1em"
              dataCy="checkout-new-version"
              on:click={() => ipc.openUrl($latestVersionInfo.announcementUrl)}>
              Check out Version
              {$latestVersionInfo.version}
            </Button>
          </div>
        {/if}
      </div>
      <div class="section-item">
        <div>Notification (Allow Upstream to make requests to the web)</div>
        <div class="action">
          <SegmentedControl
            active={$appUpdateNotificationEnabled}
            on:select={setAppUpdateNotificationEnabled}
            options={appUpdateNotificationEnabledOptions} />
        </div>
      </div>
    </section>

    <section>
      <header>
        <h3>Legal</h3>
      </header>
      <div class="section-item border">
        <div>
          <p class="typo-text-bold">Twemoji</p>
          <p style="color: var(--color-foreground-level-6);">
            Copyright 2020 Twitter, Inc and other contributors. Licensed under
            CC-BY 4.0.
          </p>
        </div>
      </div>
      <div class="section-item border">
        <div>
          <p class="typo-text-bold">Inter</p>
          <p style="color: var(--color-foreground-level-6);">
            Inter font by Rasmus Andersson licensed under the SIL Open Font
            License 1.1.
          </p>
        </div>
      </div>
      <div class="section-item">
        <div>
          <p class="typo-text-bold">Source Code Pro</p>
          <p style="color: var(--color-foreground-level-6);">
            Source Code Pro font by Adobe Fonts distributed under the SIL Open
            Font License.
          </p>
        </div>
      </div>
    </section>
  </div>
</ScreenLayout>
