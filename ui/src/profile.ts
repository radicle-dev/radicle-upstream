import { derived, Readable } from "svelte/store";

import * as error from "./error";
import * as localPeer from "./localPeer";
import * as project from "./project";
import { Status } from "./remote";
import * as remote from "./remote";
import * as waitingRoom from "./waitingRoom";
import * as proxy from "./proxy";

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
  return localPeer.requestEvents.subscribe(() => {
    fetchFollowing();
  });
});

export const following: Readable<remote.Data<Following | null>> = derived(
  [followingProjectsStore, requestedProjectsStore],
  ([follows, requests]): remote.Data<Following | null> => {
    // Transition to loading.
    if (
      follows.status === Status.Loading ||
      requests.status === Status.Loading
    ) {
      return { status: Status.Loading as const };
    }

    // Return errors.
    if (follows.status === Status.Error) {
      return follows;
    }
    if (requests.status === Status.Error) {
      return requests;
    }

    // Data loaded.
    if (
      follows.status === Status.Success &&
      requests.status === Status.Success
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
      return { status: Status.Success as const, data };
    }

    return { status: Status.NotAsked as const };
  }
);

// ACTIONS
export const fetchFollowing = (): void => {
  remote.fetch(followingProjectsStore, proxy.client.project.listTracked());
  remote.fetch(requestedProjectsStore, project.fetchSearching(), reqs => {
    return reqs.filter(req => req.type !== waitingRoom.Status.Cloned);
  });
};

export const showNotificationsForFailedProjects = async (): Promise<void> => {
  const failedProjects = await proxy.client.project.listFailed();
  failedProjects.forEach(failedProject => {
    error.show(
      new error.Error({
        code: error.Code.ProjectRequestFailure,
        message: `The project ${failedProject.metadata.name} could not be loaded`,
        details: failedProject,
      })
    );
  });
};
