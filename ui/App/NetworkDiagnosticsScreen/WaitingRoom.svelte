<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import StateTable from "./StateTable.svelte";
  import { waitingRoomEventLog, waitingRoomState } from "ui/src/localPeer";
</script>

<style>
  table {
    table-layout: fixed;
    width: 100%;
    border-collapse: collapse;
    margin-top: 1rem;
  }

  thead th:nth-child(1) {
    width: 15%;
  }

  thead th:nth-child(2) {
    width: 10%;
  }

  pre {
    overflow: scroll;
  }
</style>

<div>
  <h2>Last known waiting room state</h2>
  {#if $waitingRoomState}
    <StateTable state={$waitingRoomState} />
  {:else}
    <div>None</div>
  {/if}
  <table>
    <thead>
      <tr>
        <th>Event</th>
        <th>Timestamp</th>
        <th>State Before</th>
        <th>State After</th>
      </tr>
    </thead>
    <tbody>
      {#each $waitingRoomEventLog as transition}
        <tr>
          <td>
            <pre
              class="typo-text-small">
              {JSON.stringify(transition.event, null, 2)}
            </pre>
          </td>
          <td>{new Date(transition.timestamp).toISOString()}</td>
          <td>
            <StateTable state={transition.state_before} />
          </td>
          <td>
            <StateTable state={transition.state_after} />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
