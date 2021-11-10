<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import * as svelteStore from "ui/src/svelteStore";
  import Section from "design-system/Showcase/Section.svelte";

  import Notification from "ui/App/Notifications/Notification.svelte";
</script>

<style>
  .swatch {
    display: flex;
    margin-bottom: 2rem;
    align-items: flex-end;
  }
</style>

<h1 style="margin-bottom: 92px">Upstream specific components</h1>

<Section title="Notifications" subTitle="Info, Error and Primary">
  <div class="swatch">
    <Notification
      notification={{
        id: 1,
        type: "info",
        message: "This is an info notification",
        timesShownStore: svelteStore.writable(1),
        actions: [
          { label: "Action1", handler: () => {} },
          { label: "Action2", handler: () => {} },
        ],
      }} />
  </div>

  <div class="swatch">
    <Notification
      notification={{
        id: 1,
        type: "error",
        timesShownStore: svelteStore.writable(5),
        message: "This is an error notification",
        actions: [
          { label: "Action1", handler: () => {} },
          { label: "Action2", handler: () => {} },
        ],
      }} />
  </div>

  <div class="swatch">
    <Notification
      notification={{
        id: 1,
        type: "primary",
        timesShownStore: svelteStore.writable(3),
        message: "This is a primary notification",
        actions: [
          { label: "Action1", handler: () => {} },
          { label: "Action2", handler: () => {} },
        ],
      }} />
  </div>
</Section>
