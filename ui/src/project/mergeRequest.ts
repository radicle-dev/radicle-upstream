import * as api from "../api";
import type { Identity } from "../identity";
import type { Project } from "../project";
import * as source from "../source";
import type { Urn } from "../urn";

export interface MergeRequest {
  id: string;
  merged: boolean;
  peerId: string;
  identity?: Identity;
  title?: string;
  description?: string;
  commit: string;
}

export interface MergeRequestDetails {
  mergeRequest: MergeRequest;
  commits: source.CommitsHistory;
}

export const getAll = (projectUrn: Urn): Promise<MergeRequest[]> => {
  return api.get<MergeRequest[]>(`source/merge_requests/${projectUrn}`);
};

export const getDetails = (
  projectUrn: Urn,
  peerId: string,
  id: string
): Promise<MergeRequestDetails> => {
  return api.get<MergeRequestDetails>(`source/merge_request/${projectUrn}/`, {
    query: { peerId, id },
  });
};

export const getCommits = async (
  myPeerId: string,
  project: Project,
  mergeRequest: MergeRequest
): Promise<source.CommitsHistory> => {
  const baseProjectLatestCommit = (
    await source.fetchCommits(project.urn, myPeerId, {
      type: source.RevisionType.Branch,
      name: project.metadata.defaultBranch,
    })
  ).history[0];

  const mrCommits = await source.fetchCommits(
    project.urn,
    mergeRequest.peerId,
    {
      type: source.RevisionType.Sha,
      sha: mergeRequest.commit,
    }
  );

  const nothingNewIdx = baseProjectLatestCommit
    ? mrCommits.history.findIndex(
        ch => ch.sha1 === baseProjectLatestCommit.sha1
      )
    : 0;
  const newCommits = mrCommits.history.slice(
    0,
    nothingNewIdx === -1 ? 0 : nothingNewIdx
  );

  return {
    history: newCommits,
    stats: { ...mrCommits.stats, commits: newCommits.length },
  };
};
