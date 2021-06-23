<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import { StatusType, status } from "ui/src/localPeer";

  let isOnline = false;
  let connectedPeers: { [peerId: string]: string[] } = {};
  $: if ($status.type === StatusType.Online) {
    isOnline = true;
    connectedPeers = $status.connectedPeers;
  }
</script>

<style>
  table {
    table-layout: fixed;
    width: 100%;
    border-collapse: collapse;
    margin-top: 1rem;
  }
</style>

{#if isOnline}
  <div>
    <h2>Connected Peers</h2>
    <table>
      <thead>
        <tr>
          <td>Peer</td>
          <td>Connections</td>
        </tr>
      </thead>
      <tbody>
        {#each Object.keys(connectedPeers).sort() as peerId}
          <tr>
            <td>{peerId}</td>
            <td>
              <ul>
                {#each connectedPeers[peerId].sort() as address}
                  <li>{address}</li>
                {/each}
              </ul>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}
