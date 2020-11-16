import { derived, get, writable } from "svelte/store";
import type { Readable, Writable } from "svelte/store";
import { push } from "svelte-spa-router";

import type * as error from "../../error";
import type { HorizontalItem } from "../../menu";
import * as notification from "../../notification";
import * as path from "../../path";
import type { Project, User } from "../../project";
import * as remote from "../../remote";
import * as source from "../../source";

import IconCommit from "../../../DesignSystem/Primitive/Icon/Commit.svelte";
import IconHouse from "../../../DesignSystem/Primitive/Icon/House.svelte";

export enum ViewKind {
  Blob = "BLOB",
  Error = "ERROR",
  Root = "ROOT",
}

export interface Blob {
  kind: ViewKind.Blob;
  blob: source.Blob;
}

interface Error {
  kind: ViewKind.Error;
  error: error.Error;
}

export interface Root {
  kind: ViewKind.Root;
  readme: source.Readme | null;
}

export type View = Blob | Error | Root;

export interface Code {
  lastCommit: source.CommitHeader;
  path: string;
  view: View;
}

interface Screen {
  code: Writable<Code>;
  history: source.CommitsHistory;
  menuItems: HorizontalItem[];
  peer: User;
  project: Project;
  revisions: source.Revisions;
  selectedPath: Readable<source.SelectedPath>;
  selectedRevision: source.Branch | source.Tag;
  tree: source.Tree;
}

const pathStore = writable<source.SelectedPath>({
  request: null,
  selected: "",
});

const screenStore = remote.createStore<Screen>();
export const store = screenStore.readable;

export const fetch = async (project: Project, peer: User): Promise<void> => {
  if (!screenStore.is(remote.Status.Success)) {
    screenStore.loading();
  }

  const fethTreeRoot = async (
    selectedRevision: source.Revision
  ): Promise<[source.Tree, Code]> => {
    const tree = await source.fetchTree(
      project.urn,
      peer.peerId,
      selectedRevision,
      ""
    );
    const root = await fetchCode(project, peer, selectedRevision, tree, "");
    return [tree, root];
  };

  try {
    const revisions = await source.fetchRevisions(project.urn, peer.peerId);
    const selectedRevision = defaultRevision(project, revisions);
    const [history, [tree, root]] = await Promise.all([
      source.fetchCommits(project.urn, peer.peerId, selectedRevision),
      fethTreeRoot(selectedRevision),
    ]);

    screenStore.success({
      code: writable<Code>(root),
      history,
      menuItems: menuItems(project, history),
      peer,
      project,
      revisions,
      selectedPath: derived(pathStore, store => store),
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

    const code = await fetchCode(project, peer, selectedRevision, tree, path);
    screen.data.code.set(code);
  }
};

export const selectRevision = async (
  revision: source.Revision
): Promise<void> => {
  const current = get(screenStore);

  if (current.status === remote.Status.Success) {
    const { data } = current;
    const { code, peer, project, selectedRevision } = data;

    if (
      selectedRevision.type === revision.type &&
      selectedRevision.name === revision.name
    ) {
      return;
    }

    const fetchTreeCode = async (): Promise<[source.Tree, Code]> => {
      const tree = await source.fetchTree(
        project.urn,
        peer.peerId,
        revision,
        ""
      );

      const newCode = await fetchCode(
        project,
        peer,
        revision,
        tree,
        get(pathStore).selected
      );

      return [tree, newCode];
    };

    try {
      const [history, [tree, newCode]] = await Promise.all([
        source.fetchCommits(project.urn, peer.peerId, revision),
        fetchTreeCode(),
      ]);
      code.set(newCode);

      screenStore.success({
        ...data,
        history,
        menuItems: menuItems(project, history),
        selectedRevision: revision as source.Branch | source.Tag,
        tree,
      });
    } catch (err) {
      screenStore.error(err);
    }
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
      .catch(err => {
        commitStore.error(err);

        console.log(err.message);
        notification.error("Could not fetch commit");
      });
  }
};

export const selectCommit = (commit: source.CommitHeader): void => {
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
  path: string,
  signal: AbortSignal
): Promise<Code> => {
  const blob = await source.fetchBlob(
    project.urn,
    peer.peerId,
    path,
    revision,
    true,
    signal
  );
  return {
    lastCommit: blob.info.lastCommit,
    path,
    view: {
      kind: ViewKind.Blob,
      blob,
    },
  };
};

const fetchCode = async (
  project: Project,
  peer: User,
  revision: source.Revision,
  tree: source.Tree,
  path: string
): Promise<Code> => {
  const currentPath = get(pathStore);
  if (currentPath.request) {
    currentPath.request.abort();
  }

  const request = new AbortController();
  pathStore.set({ request, selected: path });

  let code: Code;
  try {
    if (path === "") {
      code = await fetchRoot(project, peer, revision, tree, request.signal);
    } else {
      code = await fetchBlob(project, peer, revision, path, request.signal);
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

  pathStore.set({ request: null, selected: path });

  return code;
};

const fetchRoot = async (
  project: Project,
  peer: User,
  revision: source.Revision,
  tree: source.Tree,
  signal: AbortSignal
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
        tree,
        signal
      ),
    },
  };
};

const menuItems = (
  project: Project,
  history: source.CommitsHistory
): HorizontalItem[] => {
  return [
    {
      icon: IconHouse,
      title: "Files",
      href: path.projectSourceFiles(project.urn),
      looseActiveStateMatching: true,
    },
    {
      icon: IconCommit,
      title: "Commits",
      counter: history.stats.commits,
      href: path.projectSourceCommits(project.urn),
      looseActiveStateMatching: true,
    },
  ];
};
