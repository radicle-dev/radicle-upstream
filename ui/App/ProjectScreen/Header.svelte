<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="ts">
  import type { User, Project } from "ui/src/project";

  import * as ipc from "ui/src/ipc";
  import * as modal from "ui/src/modal";
  import * as notification from "ui/src/notification";
  import * as router from "ui/src/router";
  import { LINK_URI_PREFIX } from "ui/src/customProtocolHandler";

  import Button from "design-system/Button.svelte";
  import GlobeIcon from "design-system/icons/Globe.svelte";
  import LinkIcon from "design-system/icons/Link.svelte";
  import PenIcon from "design-system/icons/Pen.svelte";
  import ThreeDotsMenu, { MenuItem } from "design-system/ThreeDotsMenu.svelte";

  import PeerSelector from "ui/App/SharedComponents/PeerSelector.svelte";
  import ManagePeersModal from "./ManagePeersModal.svelte";
  import ProjectHeader from "./ProjectHeader.svelte";

  export let project: Project;
  export let peers: User[];
  export let selectedPeer: User;
  export let selectPeer: (peer: User) => void;

  let hoverstyle: string = "";

  const mouseenter = (): void => {
    hoverstyle = "background-color: var(--color-foreground-level-2)";
  };
  const mouseleave = (): void => {
    hoverstyle = "";
  };

  const onPeerModal = (): void => {
    modal.toggle(ManagePeersModal);
  };

  let peerSelectorExpanded: boolean = false;

  function menuItems(project: Project): MenuItem[] {
    const items: MenuItem[] = [
      {
        title: "Copy link",
        icon: LinkIcon,
        event: () => {
          ipc.copyToClipboard(`${LINK_URI_PREFIX}${project.urn}`);
          notification.show({
            type: "info",
            message: "Shareable link copied to your clipboard",
          });
        },
      },
    ];

    if (project.seed) {
      const seedUrl = new URL(project.seed);
      items.push({
        title: "View in browser",
        icon: GlobeIcon,
        disabled: false,
        tooltip: undefined,
        event: () => {
          ipc.openUrl(
            `https://app.radicle.network/seeds/${seedUrl.hostname}/${project.urn}`
          );
        },
      });
    } else {
      items.push({
        title: "View in browser",
        icon: GlobeIcon,
        disabled: true,
        tooltip: "This project isn’t on a seed yet",
        event: () => {},
      });
    }

    return items;
  }
</script>

<div style="display: flex">
  <ProjectHeader
    urn={project.urn}
    name={project.metadata.name}
    description={project.metadata.description}
    onClick={() =>
      router.push({
        type: "project",
        params: {
          urn: project.urn,
          activeView: { type: "files" },
        },
      })} />

  <div style="display: flex; align-self: center; margin-left: 1.5rem;">
    <div
      style="display: flex; z-index: 10; align-items: center;"
      class:button-transition={!peerSelectorExpanded}>
      <PeerSelector
        bind:expanded={peerSelectorExpanded}
        {peers}
        on:modal={onPeerModal}
        on:select={event => selectPeer(event.detail)}
        selected={selectedPeer} />
      <Button
        ariaLabel="edit remote peers"
        dataCy="manage-remotes"
        icon={PenIcon}
        variant="outline"
        transition={false}
        on:click={onPeerModal}
        on:mouseenter={mouseenter}
        on:mouseleave={mouseleave}
        style={`margin-right: 1rem; border-top-left-radius: 0; border-bottom-left-radius: 0; padding: 0 0.5rem; ${hoverstyle}`} />
    </div>
    <ThreeDotsMenu menuItems={menuItems(project)} />
  </div>
</div>
