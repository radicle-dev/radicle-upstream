import * as api from "./api";
import * as project from "./project";
import * as remote from "./remote";

const feedStore = remote.createStore<project.Project[]>();
export const feed = feedStore.readable;

export const fetch = () =>
  api
    .get<project.Project[]>("projects/discover")
    .then(feedStore.success)
    .catch(feedStore.error);
