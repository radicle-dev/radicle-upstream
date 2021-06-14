import { get, writable } from "svelte/store";

import * as error from "./error";
import * as config from "./config";
import type * as identity from "./identity";
import * as ipc from "./ipc";
import * as remote from "./remote";
import * as source from "./source";
import type { Urn } from "./urn";
import * as validation from "./validation";
import * as proxy from "./proxy";
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
} from "./proxy/project";

export type { Metadata, Stats, Request, Peer, PeerReplicated };
export { RequestStatus, PeerReplicationStatusType, PeerRole, PeerType };

interface ConfirmedAnchor {
  type: "confirmed";
  transactionId: string;
  orgAddress: string;
  projectId: string;
  commitHash: string;
}

export interface PendingAnchor {
  type: "pending";
  confirmations: number;
  threshold: number;
  orgAddress: string;
  projectId: string;
  commitHash: string;
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

const localStateStore = remote.createStore<source.LocalState>();
export const localState = localStateStore.readable;

const projectsStore = remote.createStore<Project[]>();
export const projects = projectsStore.readable;

export const fetchList = (): void => {
  projectsStore.loading();

  proxy.client.project
    .listContributed()
    .then(projectsStore.success)
    .catch(err => projectsStore.error(error.fromUnknown(err)));
};

export const clearLocalState = (): void => {
  localStateStore.reset();
  localStateError.set("");
};

const fetchLocalState = (path: string): void => {
  localStateStore.loading();
  source
    .getLocalState(path)
    .then(localStateStore.success)
    .catch(err => localStateStore.error(error.fromUnknown(err)));
};

// NEW PROJECT
export const localStateError = writable<string>("");
export const defaultBranch = writable<string>(config.UPSTREAM_DEFAULT_BRANCH);

const projectNameMatch = "^[a-z0-9][a-z0-9._-]+$";

export const formatNameInput = (input: string): string =>
  input.replace(" ", "-");
export const extractName = (repoPath: string): string =>
  repoPath.split("/").slice(-1)[0];

// The default branches supported for preselection when importing
// a new project. Sorted by preference of preselection.
const DEFAULT_BRANCHES = [
  config.UPSTREAM_DEFAULT_BRANCH,
  config.GIT_DEFAULT_BRANCH,
];

export const defaultBranchForNewRepository = async (): Promise<string> => {
  return (
    (await ipc.getGitGlobalDefaultBranch()) || config.UPSTREAM_DEFAULT_BRANCH
  );
};

const fetchBranches = async (path: string) => {
  fetchLocalState(path);

  localStateError.set("");

  // This is just a safe guard. Since the validations on the constraints are
  // executed first, an empty path should not make it this far.
  if (!path.length) {
    return;
  }

  let state: source.LocalState;
  try {
    state = await source.getLocalState(path);
  } catch (err) {
    error.log(
      new error.Error({
        code: error.Code.LocalStateFetchFailure,
        message: err.message,
        source: err,
      })
    );
    localStateError.set(err.message);
    return;
  }

  const foundDefaultBranch = DEFAULT_BRANCHES.find(defaultBranch =>
    state.branches.includes(defaultBranch)
  );

  defaultBranch.set(foundDefaultBranch || state.branches[0]);
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
    tooShort: "Your project name should be at least 2 characters long.",
    tooLong: "Your project name should not be longer than 64 characters.",
  },
  format: {
    pattern: new RegExp(projectNameMatch, "i"),
    message:
      "Your project name has unsupported characters in it. You can only use basic letters, numbers, and the _ , - and . characters.",
  },
};

const projectDescriptionConstraints = {
  length: {
    maximum: 256,
    tooLong:
      "Your project description should not be longer than 256 characters.",
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
            "Please choose a directory that's not already a git repository.",
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

// Checks if the provided user is part of the maintainer list of the project.
export const isMaintainer = (userUrn: Urn, project: Project): boolean => {
  return project.metadata.maintainers.includes(userUrn);
};

// Checks if any of the contributors in the list is the current user.
export const isContributor = (users: User[]): boolean => {
  return !!users.find(
    u =>
      u.type === PeerType.Local &&
      (u.role === PeerRole.Maintainer || u.role == PeerRole.Contributor)
  );
};
