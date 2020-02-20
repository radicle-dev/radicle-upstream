import regexparam from "regexparam";
import { DEFAULT_PROJECT_REVISION } from "../config.js";
import { BLOB, TREE } from "./types.js";

export const search = _params => "/search";
export const feed = _params => "/feed";
export const projects = _params => "/projects";
export const projectOverview = id => `/projects/${id}/overview`;
export const projectFeed = id => `/projects/${id}/feed`;
export const projectIssues = id => `/projects/${id}/issues`;
export const projectRevisions = id => `/projects/${id}/revisions`;
export const projectFunds = id => `/projects/${id}/funds`;
export const projectSource = (id, revision, objectType, path) => {
  if (revision && path) {
    return `/projects/${id}/source/${revision}/${objectType}/${path}`;
  } else {
    return `/projects/${id}/source`;
  }
};

const PROJECT_SOURCE_PATH_MATCH = new RegExp(
  `/source/(.*)/(${BLOB}|${TREE})/(.*)`
);

export const extractProjectSourceRevision = location => {
  const rev = location.match(PROJECT_SOURCE_PATH_MATCH);
  return rev === null ? DEFAULT_PROJECT_REVISION : rev[1];
};

export const extractProjectSourceObjectPath = location => {
  const path = location.match(PROJECT_SOURCE_PATH_MATCH);
  return path === null ? "" : path[3];
};

export const extractProjectSourceObjectType = location => {
  const type = location.match(PROJECT_SOURCE_PATH_MATCH);
  return type === null ? TREE : type[2];
};

export const designSystemGuide = _params => "/design-system-guide";
export const createProject = _params => "/projects/new";
export const registerProject = id => `/projects/${id}/register`;
export const wallet = _params => "/wallet";
export const profile = _params => "/profile";
export const help = _params => "/help";

export const active = (path, location, loose = false) => {
  return regexparam(path, loose).pattern.test(location);
};
