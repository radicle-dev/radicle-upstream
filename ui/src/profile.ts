import * as project from "./project";
import * as remote from "./remote";
import * as waitingRoom from "./waitingRoom";

const followingProjectsStore = remote.createStore<project.Project[]>();
export const followingProjects = followingProjectsStore.readable;

const requestedProjectsStore = remote.createStore<
  waitingRoom.ProjectRequest[]
>();
export const requestedProjects = requestedProjectsStore.readable;

export const fetchFollowingProjects = (): void => {
  project
    .fetchTracking()
    .then(followingProjectsStore.success)
    .catch(followingProjectsStore.error);
};

export const fetchRequestedProjects = (): void => {
  project
    .fetchSearching()
    .then(requestedProjectsStore.success)
    .catch(requestedProjectsStore.error);
};
