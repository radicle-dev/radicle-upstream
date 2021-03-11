<script lang="typescript">
  import { getContext } from "svelte";
  import { push } from "svelte-spa-router";

  import * as localPeer from "../src/localPeer";
  import * as modal from "../src/modal";
  import * as path from "../src/path";
  import { isMaintainer, isContributor } from "../src/project";
  import type { User } from "../src/project";
  import { fetch, selectPeer, refresh, store } from "../src/screen/project";
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
  import PeerSelector from "../DesignSystem/Component/PeerSelector.svelte";

  import Source from "./Project/Source.svelte";

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
  const onPeerModal = () => {
    modal.toggle(path.managePeers());
  };
  const onSelectPeer = ({ detail: peer }: { detail: User }) => {
    selectPeer(peer);
  };

  localPeer.projectEvents.subscribe(event => {
    if (!event) {
      return;
    }

    if (event.urn === urn) {
      refresh();
    }
  });

  // Initialise the screen by fetching the project and associated data.
  fetch(urn);
</script>

<SidebarLayout dataCy="project-screen">
  <Remote
    {store}
    let:data={{ peerSelection, project, selectedPeer }}
    context="project-page">
    <Header.Large
      urn={project.urn}
      name={project.metadata.name}
      description={project.metadata.description}
      stats={project.stats}
      onClick={() => push(path.project(urn))}>
      <div slot="top">
        <div style="display: flex">
          <PeerSelector
            peers={peerSelection}
            on:modal={onPeerModal}
            on:open={onOpenPeer}
            on:select={onSelectPeer}
            selected={selectedPeer} />
          <Tooltip
            position={CSSPosition.Left}
            value={isMaintainer(session.identity.urn, project) ? trackTooltipMaintainer : trackTooltip}>
            <FollowToggle disabled following />
          </Tooltip>
        </div>
      </div>
    </Header.Large>
    <Source
      {project}
      {selectedPeer}
      isContributor={isContributor(peerSelection)} />
  </Remote>
</SidebarLayout>
