import { get, derived, writable, Readable } from "svelte/store";

import * as error from "../error";
import * as project from "../project";
import * as remote from "../remote";
import * as source from "../source";

interface Params {
  peer: project.User;
  project: project.Project;
  revision: source.Revision;
  path: string;
}

export enum CodeView {
  File = "FILE",
  Root = "ROOT",
  Error = "ERROR",
}

interface Shared {
  lastCommit: source.LastCommit;
}

interface Error {
  kind: CodeView.Error;
  error: error.Error;
}

interface File extends Shared {
  kind: CodeView.File;
  file: source.Blob;
  path: string;
}

interface Root extends Shared {
  kind: CodeView.Root;
  readme: source.Readme | null;
}

type Code = Error | File | Root;

const selectedPathStore = writable<string | null>(null);
export const params: Readable<remote.Data<Params>> = derived(
  [
    project.project,
    project.selectedPeer,
    project.selectedRevision,
    selectedPathStore,
  ],
  ([remoteProject, peer, revision, selectedPath], set) => {
    if (
      remoteProject.status === remote.Status.NotAsked ||
      remoteProject.status === remote.Status.Loading
    ) {
      set(remoteProject);
    }

    // We can't continue meaningfully if either of them are missing.
    if (!peer || !revision) {
      return;
    }

    if (remoteProject.status === remote.Status.Success) {
      const path = selectedPath ? selectedPath : "";
      const project = remoteProject.data;
      const current = get(params);

      if (current.status === remote.Status.Success) {
        const {
          project: { urn: currentUrn },
          peer: { peerId: currentPeerId },
          revision: currentRevision,
          path: currentPath,
        } = current.data;

        if (
          project.urn === currentUrn &&
          peer.peerId === currentPeerId &&
          revision.type === currentRevision.type &&
          (revision as source.Branch | source.Tag).name ===
            (currentRevision as source.Branch | source.Tag).name &&
          path === currentPath
        ) {
          return;
        }
      }

      set({
        status: remote.Status.Success,
        data: {
          project,
          peer,
          revision,
          path: path ? path : "",
        },
      });
    }
  },
  { status: remote.Status.NotAsked } as remote.Data<Params>
);

export const code: Readable<remote.Data<Code>> = derived(
  [params],
  ([remoteParams], set) => {
    if (
      remoteParams.status === remote.Status.NotAsked ||
      remoteParams.status === remote.Status.Loading
    ) {
      set(remoteParams);
    }

    if (remoteParams.status === remote.Status.Success) {
      const {
        data: { project, peer, revision, path },
      } = remoteParams;
      let lastCommit: source.LastCommit;

      if (path === "") {
        console.log("fetch root", peer.peerId);
        source
          .fetchObject(
            source.ObjectType.Tree,
            project.urn,
            peer.peerId,
            "",
            revision
          )
          .then(tree => {
            lastCommit = tree.info.lastCommit;

            return source.fetchReadme(
              project.urn,
              peer.peerId,
              revision,
              tree as source.Tree
            );
          })
          .then(readme => {
            console.log("success", lastCommit);
            set({
              status: remote.Status.Success,
              data: {
                kind: CodeView.Root,
                lastCommit,
                readme,
              },
            });
          });
      } else {
        source
          .fetchObject(
            source.ObjectType.Blob,
            project.urn,
            peer.peerId,
            path,
            revision
          )
          .then(blob => {
            set({
              status: remote.Status.Success,
              data: {
                kind: CodeView.File,
                lastCommit: blob.info.lastCommit,
                file: blob as source.Blob,
                path,
              },
            });
          });
      }
    }
  },
  { status: remote.Status.NotAsked } as remote.Data<Code>
);

export const selectPath = (path: string): void => {
  selectedPathStore.set(path);
};

const selectedCommitStore = writable<string | null>(null);
export const commit: Readable<remote.Data<source.Commit>> = derived(
  [project.project, selectedCommitStore],
  ([currentProject, selectedCommit], set) => {
    if (
      currentProject.status === remote.Status.NotAsked ||
      currentProject.status === remote.Status.Loading
    ) {
      set(currentProject);
    }

    if (!selectedCommit) {
      return;
    }

    if (currentProject.status === remote.Status.Success) {
      const { urn: projectUrn } = currentProject.data;

      source
        .fetchCommit(projectUrn, selectedCommit)
        .then(commit => set({ status: remote.Status.Success, data: commit }));
    }
  },
  { status: remote.Status.NotAsked } as remote.Data<source.Commit>
);

export const selectCommit = (hash: string): void => {
  selectedCommitStore.set(hash);
};

export const commits: Readable<remote.Data<source.CommitsHistory>> = derived(
  [project.project, project.selectedPeer, project.selectedRevision],
  ([currentProject, selectedPeer, selectedRevision], set) => {
    if (
      currentProject.status === remote.Status.NotAsked ||
      currentProject.status === remote.Status.Loading
    ) {
      set(currentProject);
    }

    if (!selectedPeer || !selectedRevision) {
      return;
    }

    if (currentProject.status === remote.Status.Success) {
      // TODO(xla): Only branches are supported by the underlying endpoint, this should be extended to tags as well.
      if (selectedRevision.type !== source.RevisionType.Branch) {
        return;
      }

      const { urn: projectUrn } = currentProject.data;

      source
        .fetchCommits(projectUrn, selectedPeer.peerId, selectedRevision)
        .then(commits => set({ status: remote.Status.Success, data: commits }));
    }
  },
  { status: remote.Status.NotAsked } as remote.Data<source.CommitsHistory>
);
