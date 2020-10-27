import { derived, Readable } from "svelte/store";

import * as localPeer from "./localPeer";
import * as project from "./project";
import * as remote from "./remote";
import * as waitingRoom from "./waitingRoom";

// TYPES
interface Following {
  follows: project.Project[];
  requests: waitingRoom.ProjectRequest[];
}

// STATE
const followingProjectsStore = remote.createStore<project.Project[]>();
const requestedProjectsStore = remote.createStore<
  waitingRoom.ProjectRequest[]
>();

// Subscribe to request events from the local peer to refresh the lists.
requestedProjectsStore.start(() => {
  return localPeer.requestEvents.subscribe(event => {
    fetchFollowing();
  });
});

export const following: Readable<remote.Data<Following | null>> = derived(
  [followingProjectsStore, requestedProjectsStore],
  ([follows, requests]) => {
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
          req.type !== waitingRoom.Status.Cancelled &&
          req.type !== waitingRoom.Status.TimedOut
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
  remote.fetch(followingProjectsStore, project.fetchTracking());
  remote.fetch(requestedProjectsStore, project.fetchSearching(), reqs => {
    return reqs.filter(req => req.type !== waitingRoom.Status.Cloned);
  });
};
