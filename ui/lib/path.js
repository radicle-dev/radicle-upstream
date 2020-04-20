import regexparam from "regexparam";

import { DEFAULT_PROJECT_REVISION } from "../config.js";
import { BLOB, TREE } from "../../native/types.js";

const PROJECT_SOURCE_PATH_MATCH = new RegExp(
  `/source/(.*)/(${BLOB}|${TREE})/(.*)`
);

export const search = () => "/search";
export const network = () => "/network";

export const profile = () => "/profile";
export const profileProjects = () => "/profile/projects";
export const profileWallet = () => "/profile/wallet";
export const profileSettings = () => "/profile/settings";
export const registerUser = () => "/user-registration";
export const createIdentity = _params => "/identity/new";

export const orgs = id => `/orgs/${id}`;
export const orgProjects = id => `/orgs/${id}/projects`;
export const orgFund = id => `/orgs/${id}/fund`;
export const orgMembers = id => `/orgs/${id}/members`;

export const createProject = () => "/projects/new";
export const registerProject = (projectId, registrarId) =>
  `/projects/${projectId}/register/${registrarId}`;
export const projectIssues = id => `/projects/${id}/issues`;
export const projectRevisions = id => `/projects/${id}/revisions`;
export const projectSource = (id, revision, objectType, path) => {
  if (revision && path) {
    return `/projects/${id}/source/${revision}/${objectType}/${path}`;
  } else {
    return `/projects/${id}/source`;
  }
};
export const projectCommit = (id, hash) => `/projects/${id}/commits/${hash}`;

export const transactions = id => `/transactions/${id}`;

export const designSystemGuide = () => "/design-system-guide";
export const help = () => "/help";

export const active = (path, location, loose = false) => {
  return regexparam(path, loose).pattern.test(location);
};

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
