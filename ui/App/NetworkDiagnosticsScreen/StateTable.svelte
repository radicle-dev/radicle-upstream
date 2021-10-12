<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { RoomState } from "ui/src/waitingRoom";
  import { CopyableIdentifier } from "ui/DesignSystem";

  export let state: RoomState;
</script>

<table>
  <thead>
    <tr>
      <td>Urn</td>
      <td>State</td>
      <td>Peer States</td>
    </tr>
  </thead>
  <tbody>
    {#each Object.keys(state).sort() as urn}
      <tr>
        <td>
          <CopyableIdentifier value={urn} kind="radicleId" />
        </td>
        <td>{state[urn].state}</td>
        <td>
          <table>
            <tbody>
              {#each Object.keys(state[urn].peers).sort() as peerId}
                <tr>
                  <td>
                    <CopyableIdentifier value={peerId} kind="deviceId" />
                  </td>
                  <td>{JSON.stringify(state[urn].peers[peerId])}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </td>
      </tr>
    {/each}
  </tbody>
</table>
