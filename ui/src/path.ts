import { parse, stringify, ParsedQs } from "qs";
import regexparam from "regexparam";
import { RevisionQuery } from "./source";

export const blank = (): string => "/";
export const settings = (): string => "/settings";

export const discovery = (): string => "/discovery";

export const profile = (): string => "/profile";
export const profileOnboard = (): string => "/profile/onboard";
export const profileProjects = (): string => "/profile/projects";
export const profileTracking = (): string => "/profile/tracking";
export const profileWallet = (): string => "/profile/wallet";
export const registerUser = (): string => "/user-registration";
export const onboarding = (): string => "/onboarding";

export const userProfile = (urn: string): string => `/user/${urn}`;
export const userProfileProjects = (urn: string): string =>
  `/user/${urn}/projects`;

export const orgs = (id: string): string => `/orgs/${id}`;
export const orgOnboard = (id: string): string => `/orgs/${id}/onboard`;
export const orgRegistration = (): string => `/orgs/register`;
export const orgProjects = (id: string): string => `/orgs/${id}/projects`;
export const orgFund = (id: string): string => `/orgs/${id}/fund`;
export const orgMembers = (id: string): string => `/orgs/${id}/members`;
export const memberRegistration = (id: string): string =>
  `/orgs/${id}/members/register`;

export const createProject = (): string => "/projects/new";
export const sendFunds = (): string => "/send-funds";
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
export const projectUntracked = (urn: string): string =>
  `/projects/untracked/${urn}`;

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

export const parseQueryString = (querystring: string): ParsedQs => {
  return parse(querystring);
};

export const projectCommit = (id: string, hash: string): string =>
  `/projects/${id}/commit/${hash}`;
export const projectCommits = (id: string, revision: RevisionQuery): string =>
  `/projects/${id}/commits/${JSON.stringify(revision)}`;

export const transactions = (id: string, viewerAccountId: string): string =>
  `/transactions/${id}?${stringify({ viewerAccountId })}`;

export const designSystemGuide = (): string => "/design-system-guide";

// modal routes
export const search = (): string => "/search";
export const shortcuts = (): string => "/shortcuts";

export const active = (
  path: string,
  location: string,
  loose = false
): boolean => {
  return regexparam(path, loose).pattern.test(location);
};
