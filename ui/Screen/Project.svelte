<script lang="typescript">
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as path from "../src/path";
  import { isMaintainer } from "../src/project";
  import {
    current as store,
    fetch,
    peerSelection,
    selectPeer,
    selectedPeer,
  } from "../src/screen/project";
  import type { UnsealedSession } from "../src/session";
  import { CSSPosition } from "../src/style";
  import type { Urn } from "../src/urn";

  import {
    FollowToggle,
    Header,
    Remote,
    SidebarLayout,
    Tooltip,
  } from "../DesignSystem/Component";

  import Source from "./Project/Source.svelte";
  import PeerSelector from "./Project/PeerSelector.svelte";

  export let params: { urn: Urn };

  const { urn } = params;
  const session: UnsealedSession = getContext("session");
  const trackTooltipMaintainer = "You can't unfollow your own project";
  const trackTooltip = "Unfollowing is not yet supported";

  const onOpenPeer = ({ detail: peer }: { detail: User }) => {
    if (peer.identity.urn === session.identity.urn) {
      push(path.profileProjects());
    } else {
      push(path.userProfileProjects(peer.identity.urn));
    }
  };
  const onSelectPeer = ({ detail: peer }: { detail: User }) => {
    selectPeer(peer);
  };

  // Initialise the screen by fetching the project and associated data.
  fetch(urn);
</script>

<SidebarLayout dataCy="project-screen">
  <Remote {store} let:data={{ project, peers }}>
    <Header.Large
      urn={project.urn}
      name={project.metadata.name}
      description={project.metadata.description}
      stats={project.stats}>
      <div slot="top">
        <div style="display: flex">
          <Remote store={peerSelection} let:data>
            {#if data.peers.length > 0}
              <PeerSelector
                peers={data.peers}
                on:open={onOpenPeer}
                on:select={onSelectPeer}
                selected={$selectedPeer || data.default} />
              <Tooltip
                position={CSSPosition.Left}
                value={isMaintainer(session.identity.urn, project) ? trackTooltipMaintainer : trackTooltip}>
                <FollowToggle disabled following />
              </Tooltip>
            {/if}
          </Remote>
        </div>
      </div>
    </Header.Large>
    <Source {project} />
  </Remote>
</SidebarLayout>
