import * as identity from "./identity";
import * as project from "./project";
import * as remote from "./remote";
import * as error from "./error";

const projectsStore = remote.createStore<project.Project[]>();
export const projects = projectsStore.readable;

const userStore = remote.createStore<identity.Identity>();
export const user = userStore.readable;

export const fetchProjects = (urn: string): void => {
  project
    .fetchUserList(urn)
    .then(projectsStore.success)
    .catch(err => projectsStore.error(error.fromUnknown(err)));
};

export const fetchUser = (urn: string): void => {
  identity
    .fetch(urn)
    .then(userStore.success)
    .catch(err => userStore.error(error.fromUnknown(err)));
};
