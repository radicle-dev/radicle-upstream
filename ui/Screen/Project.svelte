<script lang="typescript">
  import { onDestroy } from "svelte";

  import * as localPeer from "ui/src/localPeer";
  import * as modal from "ui/src/modal";
  import { isMaintainer, isContributor } from "ui/src/project";
  import type { User } from "ui/src/project";
  import * as router from "ui/src/router";
  import {
    fetch,
    selectPeer,
    refreshPeers,
    store,
  } from "ui/src/screen/project";
  import * as sess from "ui/src/session";
  import { CSSPosition } from "ui/src/style";
  import type { Urn } from "ui/src/urn";
  import {
    FollowToggle,
    Header,
    Remote,
    SidebarLayout,
    Tooltip,
  } from "ui/DesignSystem/Component";
  import { Button, Icon } from "ui/DesignSystem/Primitive";
  import ProjectHeader from "./Project/ProjectHeader.svelte";
  import PeerSelector from "ui/DesignSystem/Component/PeerSelector.svelte";
  import ModalManagePeers from "ui/Modal/ManagePeers.svelte";

  import Source from "./Project/Source.svelte";

  export let urn: Urn;
  export let activeView: router.ProjectView = { type: "files" };

  const session = sess.getUnsealedFromContext();
  const trackTooltipMaintainer = "You can't unfollow your own project";
  const trackTooltip = "Unfollowing is not yet supported";

  const onOpenPeer = ({ detail: peer }: { detail: User }) => {
    if (peer.identity.urn === session.identity.urn) {
      router.push({ type: "profile", activeTab: "projects" });
    } else {
      router.push({
        type: "userProfile",
        activeTab: "projects",
        urn: peer.identity.urn,
      });
    }
  };
  const onPeerModal = () => {
    modal.toggle(ModalManagePeers);
  };
  const onSelectPeer = ({ detail: peer }: { detail: User }) => {
    selectPeer(peer);
  };

  const unsubscribeFromProjectEvents = localPeer.projectEvents.onValue(
    event => {
      if (event.urn === urn) {
        refreshPeers();
      }
    }
  );
  onDestroy(unsubscribeFromProjectEvents);

  // Initialise the screen by fetching the project and associated data.
  fetch(urn);
</script>

<SidebarLayout dataCy="project-screen">
  <Remote {store} let:data={{ peerSelection, project, selectedPeer }}>
    <Header>
      <ProjectHeader
        slot="left"
        urn={project.urn}
        name={project.metadata.name}
        description={project.metadata.description}
        stats={project.stats}
        onClick={() =>
          router.push({
            type: "project",
            urn: urn,
            activeView: { type: "files" },
          })} />

      <div slot="right" style="display: flex;">
        <PeerSelector
          peers={peerSelection}
          on:modal={onPeerModal}
          on:open={onOpenPeer}
          on:select={onSelectPeer}
          selected={selectedPeer} />
        <Button
          dataCy="manage-remotes"
          icon={Icon.Pen}
          variant="outline"
          style="margin-right: 1rem;"
          on:click={onPeerModal}>
          Add remotes
        </Button>
        <Tooltip
          position={CSSPosition.Left}
          value={isMaintainer(session.identity.urn, project)
            ? trackTooltipMaintainer
            : trackTooltip}>
          <FollowToggle disabled following />
        </Tooltip>
      </div>
    </Header>
    <Source
      {activeView}
      {project}
      {selectedPeer}
      isContributor={isContributor(peerSelection)} />
  </Remote>
</SidebarLayout>
