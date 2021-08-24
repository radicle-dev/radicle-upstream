// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import { derived, Readable } from "svelte/store";

import * as error from "./error";
import * as localPeer from "./localPeer";
import * as project from "./project";
import * as remote from "./remote";
import * as proxy from "./proxy";

// TYPES
interface Following {
  follows: project.Project[];
  requests: project.Request[];
}

// STATE
const followingProjectsStore = remote.createStore<project.Project[]>();
const requestedProjectsStore = remote.createStore<project.Request[]>();

// Subscribe to request events from the local peer to refresh the lists.
requestedProjectsStore.start(() => {
  return localPeer.requestEvents.subscribe(() => {
    fetchFollowing();
  });
});

export const following: Readable<remote.Data<Following | null>> = derived(
  [followingProjectsStore, requestedProjectsStore],
  ([follows, requests]): remote.Data<Following | null> => {
    // Transition to loading.
    if (
      follows.status === remote.Status.Loading ||
      requests.status === remote.Status.Loading
    ) {
      return { status: remote.Status.Loading };
    }

    // Return errors.
    if (follows.status === remote.Status.Error) {
      return follows;
    }
    if (requests.status === remote.Status.Error) {
      return requests;
    }

    // Data loaded.
    if (
      follows.status === remote.Status.Success &&
      requests.status === remote.Status.Success
    ) {
      let data = null;
      const reqs = requests.data.filter(
        req =>
          req.type !== project.RequestStatus.Cancelled &&
          req.type !== project.RequestStatus.TimedOut
      );

      if (follows.data.length > 0 || reqs.length > 0) {
        data = {
          follows: follows.data,
          requests: reqs,
        };
      }
      return { status: remote.Status.Success, data };
    }

    return { status: remote.Status.NotAsked };
  }
);

// ACTIONS
export const fetchFollowing = (): void => {
  remote.fetch(followingProjectsStore, proxy.client.project.listTracked());
  remote.fetch(
    requestedProjectsStore,
    proxy.client.project
      .requestsList()
      .then(reqs =>
        reqs.filter(req => req.type !== project.RequestStatus.Cloned)
      )
  );
};

export const showNotificationsForFailedProjects = async (): Promise<void> => {
  try {
    const failedProjects = await proxy.client.project.listFailed();
    failedProjects.forEach(failedProject => {
      error.show(
        new error.Error({
          code: error.Code.ProjectRequestFailure,
          message: `The project ${failedProject.metadata.name} couldn’t be loaded`,
          details: failedProject,
        })
      );
    });
  } catch (err: unknown) {
    error.show(
      new error.Error({
        code: error.Code.ProjectRequestFailure,
        message: "Failed to get failed projects",
        source: err,
      })
    );
  }
};
