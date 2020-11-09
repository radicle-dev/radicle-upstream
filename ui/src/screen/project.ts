import { derived, writable, Readable } from "svelte/store";

import * as error from "../error";
import * as project from "../project";
import * as remote from "../remote";
import * as source from "../source";

export enum CodeView {
  File = "FILE",
  Root = "ROOT",
}

interface Shared {
  peer: project.User;
  revision: source.Revision;
}

interface File extends Shared {
  kind: CodeView.File;
  file: source.Blob | error.Error;
  project: project.Project;
}

interface Root extends Shared {
  kind: CodeView.Root;
  lastCommit: source.LastCommit;
  project: project.Project;
  readme: source.Readme | null;
}

type Code = File | Root;

export const code: Readable<remote.Data<Code>> = derived(
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
      const { urn: projectUrn } = currentProject.data;
      let lastCommit: source.LastCommit;

      source
        .fetchObject(
          source.ObjectType.Tree,
          projectUrn,
          selectedPeer.peerId,
          "",
          selectedRevision
        )
        .then(tree => {
          lastCommit = tree.info.lastCommit;

          return source.readme(
            projectUrn,
            selectedPeer.peerId,
            selectedRevision,
            tree as source.Tree
          );
        })
        .then(readme => {
          set({
            status: remote.Status.Success,
            data: {
              kind: CodeView.Root,
              lastCommit,
              peer: selectedPeer,
              project: currentProject.data,
              readme: readme,
              revision: selectedRevision,
            },
          });
        });
    }
  },
  { status: remote.Status.NotAsked } as remote.Data<Code>
);

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
