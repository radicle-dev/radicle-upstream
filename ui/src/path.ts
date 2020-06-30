import regexparam from "regexparam";

import { ObjectType } from "./source";

const PROJECT_SOURCE_PATH_MATCH = new RegExp(
  `/source/(.*)/(.*)/(${ObjectType.Blob}|${ObjectType.Tree})?/?(.*)?`
);

export const search = (): string => "/search";
export const settings = (): string => "/settings";

export const profile = (): string => "/profile";
export const profileOnboard = (): string => "/profile/onboard";
export const profileProjects = (): string => "/profile/projects";
export const profileWallet = (): string => "/profile/wallet";
export const registerUser = (): string => "/user-registration";
export const createIdentity = (): string => "/identity/new";

export const orgs = (id: string): string => `/orgs/${id}`;
export const orgOnboard = (id: string): string => `/orgs/${id}/onboard`;
export const orgRegistration = (): string => `/orgs/register`;
export const orgProjects = (id: string): string => `/orgs/${id}/projects`;
export const orgFund = (id: string): string => `/orgs/${id}/fund`;
export const orgMembers = (id: string): string => `/orgs/${id}/members`;
export const memberRegistration = (id: string): string =>
  `/orgs/${id}/members/register`;

export const createProject = (): string => "/projects/new";
export const registerProject = (domainId: string): string =>
  `/projects/register/${domainId}`;
export const registerExistingProject = (
  projectId: string,
  domainId: string
): string => `/projects/${projectId}/register/${domainId}`;
export const projectIssues = (id: string): string => `/projects/${id}/issues`;
export const projectIssue = (id: string): string => `/projects/${id}/issue`;
export const projectRevisions = (id: string): string =>
  `/projects/${id}/revisions`;
export const projectSource = (
  id: string,
  userId: string,
  revision: string,
  objectType: string,
  objectPath: string
): string => {
  if (revision && objectType && objectPath) {
    return `/projects/${id}/source/${userId}/${revision}/${objectType}/${
      objectType === ObjectType.Tree ? `${objectPath}/` : objectPath
    }`;
  } else if (revision && objectType) {
    return `/projects/${id}/source/${userId}/${revision}/${objectType}`;
  } else if (revision) {
    return `/projects/${id}/source/${userId}/${revision}`;
  } else {
    return `/projects/${id}/source/${userId}`;
  }
};
export const projectCommit = (id: string, user: string, hash: string): string =>
  `/projects/${id}/${user}/commit/${hash}`;
export const projectCommits = (
  id: string,
  user: string,
  revision: string
): string => `/projects/${id}/${user}/commits/${encodeURIComponent(revision)}`;

export const transactions = (id: string): string => `/transactions/${id}`;

export const designSystemGuide = (): string => "/design-system-guide";
export const help = (): string => "/help";

export const active = (
  path: string,
  location: string,
  loose = false
): boolean => {
  return regexparam(path, loose).pattern.test(location);
};

export const extractProjectSourceUser = (
  location: string,
  fallback = ""
): string => {
  const user = PROJECT_SOURCE_PATH_MATCH.exec(location);
  return user === null ? fallback : user[1];
};

export const extractProjectSourceRevision = (
  location: string,
  fallback = ""
): string => {
  const rev = PROJECT_SOURCE_PATH_MATCH.exec(location);
  console.log("bla: ");
  console.log(rev);
  return rev === null ? fallback : rev[2];
};

export const extractProjectSourceObjectPath = (
  location: string,
  fallback = ""
): string => {
  const path = PROJECT_SOURCE_PATH_MATCH.exec(location);
  return path === null ? fallback : path[4];
};

export const extractProjectSourceObjectType = (
  location: string,
  fallback: ObjectType = ObjectType.Tree
): string => {
  const type = PROJECT_SOURCE_PATH_MATCH.exec(location);
  return type === null ? fallback : type[3];
};
