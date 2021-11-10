<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import dayjs from "dayjs";
  import lodash from "lodash";

  import { notificationHistory } from "ui/src/notification";
  import Json from "./Json.svelte";

  // eslint-disable-next-line @typescript-eslint/ban-types
  function hasOwnProperty<X extends {}, Y extends PropertyKey>(
    obj: X,
    prop: Y
  ): obj is X & Record<Y, unknown> {
    // eslint-disable-next-line no-prototype-builtins
    return obj.hasOwnProperty(prop);
  }
</script>

<style>
  .container {
    gap: 2rem;
    display: flex;
    flex-direction: column;
  }

  .notification {
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-2);
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 0.5rem;
    padding: 1rem;
  }

  .list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  pre {
    font-family: var(--typeface-mono-regular);
    font-size: 14px;
    background-color: var(--color-foreground-level-1);
    border-radius: 0.5rem;
    padding: 1rem;
    overflow: scroll;
  }
</style>

<div class="container">
  {#if $notificationHistory.length > 0}
    <div class="list">
      {#each lodash
        .orderBy($notificationHistory, notification => notification.timestamp)
        .reverse() as notification}
        <div class="notification">
          <div
            style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
            <h5>
              {notification.type}
            </h5>
            <h5>
              {dayjs(notification.timestamp).format(
                "HH:mm:ss.SSS · DD.MM.YYYY"
              )}
            </h5>
          </div>

          <div class="typo-text">{notification.message}</div>
          {#if notification.details}
            {#if typeof notification.details === "object" && notification.details !== null && hasOwnProperty(notification.details, "stack") && typeof notification.details.stack === "string"}
              <Json data={lodash.omit(notification.details, ["stack"])} />
              <pre>{notification.details.stack}</pre>
            {:else}
              <Json data={notification.details} />
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>
