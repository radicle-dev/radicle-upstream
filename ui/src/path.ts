import regexparam from "regexparam";

import * as config from "./config";
import { ObjectType } from "./source";

const PROJECT_SOURCE_PATH_MATCH = new RegExp(
  `/source/(.*)/(${ObjectType.Blob}|${ObjectType.Tree})/(.*)`
);

export const search = (): string => "/search";
export const network = (): string => "/network";

export const profile = (): string => "/profile";
export const profileOnboard = (): string => "/profile/onboard";
export const profileProjects = (): string => "/profile/projects";
export const profileWallet = (): string => "/profile/wallet";
export const profileSettings = (): string => "/profile/settings";
export const registerUser = (): string => "/user-registration";
export const createIdentity = (): string => "/identity/new";

export const orgs = (id: string): string => `/orgs/${id}`;
export const orgRegistration = (): string => `/orgs/register`;
export const orgProjects = (id: string): string => `/orgs/${id}/projects`;
export const orgFund = (id: string): string => `/orgs/${id}/fund`;
export const orgMembers = (id: string): string => `/orgs/${id}/members`;

export const createProject = (): string => "/projects/new";
export const registerProject = (registrarId: string): string =>
  `/projects/register/${registrarId}`;
export const registerExistingProject = (
  projectId: string, 
  registrarId: string,
): string =>
  `/projects/${projectId}/register/${registrarId}`;
export const projectIssues = (id: string): string => `/projects/${id}/issues`;
export const projectRevisions = (id: string): string => `/projects/${id}/revisions`;
export const projectSource = (
  id: string,
  revision: string,
  objectType: string,
  path: string,
): string => {
  if (revision && path) {
    return `/projects/${id}/source/${revision}/${objectType}/${
      objectType === ObjectType.Tree ? `${path}/` : path
    }`;
  } else {
    return `/projects/${id}/source`;
  }
};
export const projectCommit = (id: string, hash: string): string =>
  `/projects/${id}/commit/${hash}`;
export const projectCommits = (id: string, branch: string): string =>
  `/projects/${id}/commits/${branch}`;

export const transactions = (id: string): string => `/transactions/${id}`;

export const designSystemGuide = (): string => "/design-system-guide";
export const help = (): string => "/help";

export const active = (path: string, location: string, loose = false): boolean => {
  return regexparam(path, loose).pattern.test(location);
};

export const extractProjectSourceRevision = (location: string): string => {
  const rev = PROJECT_SOURCE_PATH_MATCH.exec(location);
  return rev === null ? config.DEFAULT_PROJECT_REVISION : rev[1];
};

export const extractProjectSourceObjectPath = (location: string): string => {
  const path = PROJECT_SOURCE_PATH_MATCH.exec(location);
  return path === null ? "" : path[3];
};

export const extractProjectSourceObjectType = (location: string): string => {
  const type = PROJECT_SOURCE_PATH_MATCH.exec(location);
  return type === null ? ObjectType.Tree : type[2];
};
