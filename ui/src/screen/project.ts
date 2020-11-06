import { derived, Readable } from "svelte/store";

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
  readme: Blob | null;
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
      source
        .fetchObject(
          source.ObjectType.Tree,
          currentProject.data.urn,
          selectedPeer.peerId,
          "",
          selectedRevision
        )
        .then(obj =>
          set({
            status: remote.Status.Success,
            data: {
              kind: CodeView.Root,
              lastCommit: obj.info.lastCommit,
              peer: selectedPeer,
              project: currentProject.data,
              readme: null,
              revision: selectedRevision,
            },
          })
        )
        .catch(err => set({ status: remote.Status.Error, error: err }));
    }
  },
  { status: remote.Status.NotAsked } as remote.Data<Code>
);
