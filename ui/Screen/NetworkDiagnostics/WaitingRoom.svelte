<script lang="typescript">
  import JSONTree from "svelte-json-tree";

  import StateTable from "./StateTable.svelte";
  import { waitingRoomEventLog, waitingRoomState } from "ui/src/localPeer";
</script>

<style>
  table {
    table-layout: fixed;
    width: 100%;
    border-collapse: collapse;
    border: 3px solid purple;
  }

  thead th:nth-child(1) {
    width: 15%;
  }

  thead th:nth-child(2) {
    width: 10%;
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
            <JSONTree value={transition.event} />
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
