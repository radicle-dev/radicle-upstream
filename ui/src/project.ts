// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { get, writable } from "svelte/store";

import * as proxy from "ui/src/proxy";
import type { LocalState } from "proxy-client/source";
import type * as ensResolver from "./org/ensResolver";
import * as error from "./error";
import type * as identity from "./identity";
import * as ipc from "./ipc";
import * as remote from "./remote";
import * as validation from "./validation";
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
  peerId: identity.PeerId;
  type: PeerType;
  identity: identity.Identity;
  role: PeerRole;
}

const creationStore = remote.createStore<Project>();
export const creation = creationStore.readable;

const localStateStore = remote.createStore<LocalState>();
export const localState = localStateStore.readable;

export const clearLocalState = (): void => {
  localStateStore.reset();
  localStateError.set("");
};

const fetchLocalState = (path: string): void => {
  remote.fetch(localStateStore, proxy.client.source.localStateGet(path));
};

export const UPSTREAM_DEFAULT_BRANCH = "main";
const GIT_DEFAULT_BRANCH = "master";

// NEW PROJECT
export const localStateError = writable<string>("");
export const defaultBranch = writable<string>(UPSTREAM_DEFAULT_BRANCH);

const projectNameMatch = "^[a-z0-9][a-z0-9._-]+$";

export const formatNameInput = (input: string): string =>
  input.replace(" ", "-");
export const extractName = (repoPath: string): string =>
  repoPath.split("/").slice(-1)[0];

// The default branches supported for preselection when importing
// a new project. Sorted by preference of preselection.
const DEFAULT_BRANCHES = [UPSTREAM_DEFAULT_BRANCH, GIT_DEFAULT_BRANCH];

export const defaultBranchForNewRepository = async (): Promise<string> => {
  return (await ipc.getGitGlobalDefaultBranch()) || UPSTREAM_DEFAULT_BRANCH;
};

async function fetchBranches(path: string): Promise<void> {
  fetchLocalState(path);

  localStateError.set("");

  // This is just a safe guard. Since the validations on the constraints are
  // executed first, an empty path should not make it this far.
  if (!path.length) {
    return;
  }

  let state: LocalState;
  try {
    state = await proxy.client.source.localStateGet(path);
  } catch (unknownErr: unknown) {
    const err = error.fromUnknown(
      unknownErr,
      error.Code.LocalStateFetchFailure
    );
    error.log(err);
    localStateError.set(err.message);
    return;
  }

  const foundDefaultBranch = DEFAULT_BRANCHES.find(defaultBranch =>
    state.branches.includes(defaultBranch)
  );

  defaultBranch.set(foundDefaultBranch || state.branches[0]);
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

const validateExistingRepository = (path: string): Promise<boolean> => {
  return fetchBranches(path).then(() => {
    return (
      !get(localStateError).match("could not find repository") &&
      !get(localStateError).match("repository has no branches")
    );
  });
};

const validateNewRepository = async (path: string): Promise<boolean> => {
  await fetchBranches(path);
  return !!get(localStateError).match("could not find repository");
};

const projectNameConstraints = {
  presence: {
    message: "You must provide a project name",
    allowEmpty: false,
  },
  firstHandleChar: {
    valueName: "project name",
  },
  length: {
    minimum: 2,
    maximum: 64,
    tooShort:
      "Oops, your project’s name needs to be at least 2 characters long.",
    tooLong: "Oh, your project’s name can’t have more than 64 characters.",
  },
  format: {
    pattern: new RegExp(projectNameMatch, "i"),
    message:
      "Your project’s name has some characters that aren’t supported. You can only use basic letters, numbers, and the _ , - and . characters.",
  },
};

const projectDescriptionConstraints = {
  length: {
    maximum: 256,
    tooLong:
      "Whoa Shakespeare, your project’s description can’t be longer than 256 characters. Shorten it a bit!",
  },
};

export const nameValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(projectNameConstraints);
};

export const descriptionValidationStore = (): validation.ValidationStore => {
  return validation.createValidationStore(projectDescriptionConstraints);
};

export const repositoryPathValidationStore = (
  newRepository: boolean
): validation.ValidationStore => {
  if (newRepository) {
    return validation.createValidationStore(
      {
        presence: {
          message: "Pick a directory for the new project",
          allowEmpty: false,
        },
      },
      [
        {
          promise: validateNewRepository,
          validationMessage:
            "Please choose a directory that’s not already a git repository.",
        },
      ]
    );
  } else {
    return validation.createValidationStore(
      {
        presence: {
          message: "Pick a directory with an existing repository",
          allowEmpty: false,
        },
      },
      [
        {
          promise: validateExistingRepository,
          validationMessage:
            "The directory should contain a git repository with at least one branch",
        },
      ]
    );
  }
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
