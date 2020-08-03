import * as api from "./api";
import * as project from "./project";
import * as remote from "./remote";

const projectsStore = remote.createStore<project.Project[]>();
export const projects = projectsStore.readable;

export const fetch = () =>
  api
    .get<project.Project[]>("projects/discover")
    .then(projectsStore.success)
    .catch(projectsStore.error);
