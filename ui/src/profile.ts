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
  remote.fetch(followingProjectsStore, project.fetchTracking());
};

export const fetchRequestedProjects = (): void => {
  remote.fetch(requestedProjectsStore, project.fetchSearching(), reqs => {
    return reqs.filter(req => req.type !== waitingRoom.Status.Cloned);
  });
};
