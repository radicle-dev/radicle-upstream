import { derived, get, writable } from "svelte/store";
import type { Readable, Writable } from "svelte/store";
import { push } from "svelte-spa-router";

import type * as error from "../../error";
import * as path from "../../path";
import type { Project, User } from "../../project";
import * as remote from "../../remote";
import * as source from "../../source";

export enum ViewKind {
  Blob = "BLOB",
  Error = "ERROR",
  Root = "ROOT",
}

interface Blob {
  kind: ViewKind.Blob;
  blob: source.Blob;
}

interface Error {
  kind: ViewKind.Error;
  error: error.Error;
}

interface Root {
  kind: ViewKind.Root;
  readme: source.Readme | null;
}

type View = Blob | Error | Root;

export interface Code {
  lastCommit: source.LastCommit;
  path: string;
  view: View;
}

interface Screen {
  code: Writable<Code>;
  history: source.CommitsHistory;
  path: Readable<string>;
  peer: User;
  project: Project;
  revisions: source.Revisions;
  selectedRevision: source.Branch | source.Tag;
  tree: source.Tree;
}

const pathStore = writable<string>("");

const screenStore = remote.createStore<Screen>();
export const store = screenStore.readable;

export const fetch = async (project: Project, peer: User): Promise<void> => {
  if (!screenStore.is(remote.Status.Success)) {
    screenStore.loading();
  }

  try {
    const revisions = await source.fetchRevisions(project.urn, peer.peerId);
    const selectedRevision = defaultRevision(project, revisions);
    const [history, tree] = await Promise.all([
      source.fetchCommits(project.urn, peer.peerId, selectedRevision),
      source.fetchTree(project.urn, peer.peerId, selectedRevision, ""),
    ]);
    const root = await fetchRoot(project, peer, selectedRevision, tree);

    screenStore.success({
      code: writable<Code>(root),
      path: derived(pathStore, store => store),
      history,
      peer,
      project,
      revisions,
      selectedRevision,
      tree,
    });
  } catch (err) {
    screenStore.error(err);
  }
};

export const selectPath = async (path: string): Promise<void> => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const {
      data: { peer, project, selectedRevision, tree },
    } = screen;

    pathStore.set(path);

    let code: Code;
    try {
      if (path === "") {
        code = await fetchRoot(project, peer, selectedRevision, tree);
      } else {
        code = await fetchBlob(project, peer, selectedRevision, path);
      }
    } catch (err) {
      code = {
        lastCommit: tree.info.lastCommit,
        path,
        view: {
          kind: ViewKind.Error,
          error: err,
        },
      };
    }

    screen.data.code.set(code);
  }
};

export const selectRevision = (revision: source.Revision): void => {
  const current = get(screenStore);

  if (current.status === remote.Status.Success) {
    const { data } = current;
    const { peer, project } = data;

    Promise.all([
      source.fetchCommits(project.urn, peer.peerId, revision as source.Branch),
      source.fetchTree(project.urn, peer.peerId, revision, ""),
    ])
      .then(([history, tree]) => {
        screenStore.success({
          ...data,
          history,
          selectedRevision: revision as source.Branch | source.Tag,
          tree,
        });
      })
      .catch(screenStore.error);
  }
};

const commitStore = remote.createStore<source.Commit>();
export const commit = commitStore.readable;

export const fetchCommit = (sha1: string): void => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const {
      data: { project },
    } = screen;

    source
      .fetchCommit(project.urn, sha1)
      .then(commitStore.success)
      .catch(commitStore.error);
  }
};

export const selectCommit = (commit: source.Commit): void => {
  const screen = get(screenStore);

  if (screen.status === remote.Status.Success) {
    const {
      data: { project },
    } = screen;
    push(path.projectSourceCommit(project.urn, commit.sha1));
  }
};

const defaultRevision = (
  project: Project,
  revisions: source.Revisions
): source.Branch => {
  const projectDefault = revisions.branches.find(
    (branch: source.Branch) => branch.name === project.metadata.defaultBranch
  );
  return projectDefault ? projectDefault : revisions.branches[0];
};

const fetchBlob = async (
  project: Project,
  peer: User,
  revision: source.Revision,
  path: string
): Promise<Code> => {
  const blob = await source.fetchObject(
    source.ObjectType.Blob,
    project.urn,
    peer.peerId,
    path,
    revision
  );
  return {
    lastCommit: blob.info.lastCommit,
    path,
    view: {
      kind: ViewKind.Blob,
      blob: blob as source.Blob,
    },
  };
};

const fetchRoot = async (
  project: Project,
  peer: User,
  revision: source.Revision,
  tree: source.Tree
): Promise<Code> => {
  return {
    lastCommit: tree.info.lastCommit,
    path: "",
    view: {
      kind: ViewKind.Root,
      readme: await source.fetchReadme(
        project.urn,
        peer.peerId,
        revision,
        tree
      ),
    },
  };
};

/* export const code: Readable<remote.Data<Code>> = derived( */
/*   [params], */
/*   ([remoteParams], set) => { */
/*     if ( */
/*       remoteParams.status === remote.Status.NotAsked || */
/*       remoteParams.status === remote.Status.Loading */
/*     ) { */
/*       set(remoteParams); */
/*     } */

/*     if (remoteParams.status === remote.Status.Success) { */
/*       const { */
/*         data: { project, peer, revision, path }, */
/*       } = remoteParams; */
/*       let lastCommit: source.LastCommit; */

/*       if (path === "") { */
/*         source */
/*           .fetchObject( */
/*             source.ObjectType.Tree, */
/*             project.urn, */
/*             peer.peerId, */
/*             "", */
/*             revision */
/*           ) */
/*           .then(tree => { */
/*             lastCommit = tree.info.lastCommit; */

/*             return source.fetchReadme( */
/*               project.urn, */
/*               peer.peerId, */
/*               revision, */
/*               tree as source.Tree */
/*             ); */
/*           }) */
/*           .then(readme => { */
/*             set({ */
/*               status: remote.Status.Success, */
/*               data: { */
/*                 kind: CodeView.Root, */
/*                 lastCommit, */
/*                 readme, */
/*               }, */
/*             }); */
/*           }); */
/*       } else { */
/*         source */
/*           .fetchObject( */
/*             source.ObjectType.Blob, */
/*             project.urn, */
/*             peer.peerId, */
/*             path, */
/*             revision */
/*           ) */
/*           .then(blob => { */
/*             set({ */
/*               status: remote.Status.Success, */
/*               data: { */
/*                 kind: CodeView.File, */
/*                 lastCommit: blob.info.lastCommit, */
/*                 file: blob as source.Blob, */
/*                 path, */
/*               }, */
/*             }); */
/*           }) */
/*           .catch(err => { */
/*             set({ */
/*               status: remote.Status.Success, */
/*               data: { */
/*                 kind: CodeView.Error, */
/*                 error: err, */
/*               }, */
/*             }); */
/*           }); */
/*       } */
/*     } */

/*     return (): void => { */
/*       set({ status: remote.Status.NotAsked }); */
/*     }; */
/*   }, */
/*   { status: remote.Status.NotAsked } as remote.Data<Code> */
/* ); */
