import { parse, stringify } from "qs";
import regexparam from "regexparam";
import { RevisionQuery } from "./source";

export const search = (): string => "/search";
export const settings = (): string => "/settings";

export const discovery = (): string => "/discovery";

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
  projectId: string,
  peerId?: string,
  revision?: RevisionQuery,
  objectType?: string,
  objectPath?: string
): string => {
  return `/projects/${projectId}/source?${stringify({
    peerId,
    revision,
    objectType,
    objectPath,
  })}`;
};

export const parseProjectSourceLocation = (querystring: string) => {
  return parse(querystring);
};

export const projectCommit = (id: string, hash: string): string =>
  `/projects/${id}/commit/${hash}`;
export const projectCommits = (id: string, revision: RevisionQuery): string =>
  `/projects/${id}/commits/${revision}`;

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
