// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import type * as ensResolver from "./org/ensResolver";
import type * as identity from "./identity";
import {
  Project as ProxyProject,
  Metadata,
  Stats,
  Request,
  RequestStatus,
  Peer,
  PeerType,
  PeerRole,
  PeerReplicationStatusType,
  PeerReplicated,
} from "proxy-client/project";

export type { Metadata, Stats, Request, Peer, PeerReplicated };
export { RequestStatus, PeerReplicationStatusType, PeerRole, PeerType };

export interface ConfirmedAnchor {
  type: "confirmed";
  transactionId: string;
  orgAddress: string;
  projectId: string;
  commitHash: string;
  timestamp: number;
  registration?: ensResolver.Registration;
}

export interface PendingAnchor {
  type: "pending";
  confirmations: number;
  threshold: number;
  orgAddress: string;
  projectId: string;
  commitHash: string;
  timestamp: number;
  registration?: ensResolver.Registration;
}

export type Anchor = ConfirmedAnchor | PendingAnchor;

export interface Project extends ProxyProject {
  anchor?: Anchor;
}
export interface User {
  peerId: string;
  type: PeerType;
  identity: identity.Identity;
  role: PeerRole;
}

// Creates a sorted user list from a peer list.
//
// * Filters out peers that are not replicated
// * Only includes the local peer if we forked the project
// * Sorts the list with the local peer at the beginning, then
// delegates, then contributors, then trackers.
export const userList = (peers: Peer[]): User[] => {
  return peers
    .map(peer => {
      if (peer.status.type !== PeerReplicationStatusType.Replicated) {
        return undefined;
      }

      if (
        peer.type === PeerType.Local &&
        peer.status.role === PeerRole.Tracker
      ) {
        return undefined;
      }

      return {
        type: peer.type,
        peerId: peer.peerId,
        identity: peer.status.user,
        role: peer.status.role,
      };
    })
    .filter((user): user is User => user !== undefined)
    .sort((a, b) => {
      if (a.role === PeerRole.Delegate && b.role !== PeerRole.Delegate) {
        return -1;
      }
      if (a.role !== PeerRole.Delegate && b.role === PeerRole.Delegate) {
        return 1;
      }

      if (a.role === PeerRole.Contributor && b.role === PeerRole.Tracker) {
        return -1;
      }
      if (a.role === PeerRole.Tracker && b.role === PeerRole.Contributor) {
        return 1;
      }

      return 0;
    })
    .sort((a, b) => {
      if (a.type === PeerType.Local && b.type === PeerType.Remote) {
        return -1;
      }
      if (a.type === PeerType.Remote && b.type === PeerType.Local) {
        return 1;
      }

      return 0;
    });
};

// Checks if the provided user is part of the delegate list of the project.
export const isDelegate = (userUrn: string, project: Project): boolean => {
  return project.metadata.delegates.includes(userUrn);
};

// Checks if any of the contributors in the list is the current user.
export const isContributor = (users: User[]): boolean => {
  return !!users.find(
    u =>
      u.type === PeerType.Local &&
      (u.role === PeerRole.Delegate || u.role === PeerRole.Contributor)
  );
};
