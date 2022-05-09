<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import dayjs from "dayjs";
  import lodash from "lodash";

  import { waitingRoomEventLog, waitingRoomState } from "ui/src/localPeer";
  import Loading from "ui/App/SharedComponents/Loading.svelte";
  import Json from "./Json.svelte";
</script>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .transition {
    border-radius: 0.5rem;
    border: 1px solid var(--color-foreground-level-2);
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 0.5rem;
    padding: 1rem;
  }

  .states {
    display: flex;
    gap: 1rem;
  }
</style>

<div class="container">
  {#if $waitingRoomState !== null && $waitingRoomEventLog !== []}
    <Json title="Latest state" data={$waitingRoomState} />

    <div>
      <h5>State transitions</h5>
      <div class="container">
        {#each lodash
          .orderBy($waitingRoomEventLog, event => event.timestamp)
          .reverse() as transition}
          <div class="transition">
            <div>
              <h5>
                {dayjs(transition.timestamp).format(
                  "DD.MM.YYYY / HH:mm:ss.SSS"
                )}
              </h5>
            </div>

            <div>
              <h5>Event</h5>
              <Json data={transition.event} />
            </div>

            <div class="states">
              <Json title="State before" data={transition.state_before} />
              <Json title="State after" data={transition.state_after} />
            </div>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <Loading
      style="height: calc(100vh - var(--bigheader-height) - var(--topbar-height));"
      ><p style:color="var(--color-foreground-level-6)">
        Waiting for an event…
      </p></Loading>
  {/if}
</div>
