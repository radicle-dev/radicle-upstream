<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import type { RoomState } from "ui/src/waitingRoom";
  import StyledCopyable from "ui/DesignSystem/StyledCopyable.svelte";

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
          <StyledCopyable value={urn} truncate={true} expandable={false} />
        </td>
        <td>{state[urn].state}</td>
        <td>
          <table>
            <tbody>
              {#each Object.keys(state[urn].peers).sort() as peerId}
                <tr>
                  <td>
                    <StyledCopyable
                      value={peerId}
                      truncate={true}
                      expandable={false} />
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
