import { parse, ParsedQs } from "qs";
import regexparam from "regexparam";

export const blank = (): string => "/";
export const settings = (): string => "/settings";

export const discovery = (): string => "/discovery";

export const profile = (): string => "/profile";
export const profileOnboard = (): string => "/profile/onboard";
export const profileProjects = (): string => "/profile/projects";
export const profileFollowing = (): string => "/profile/following";
export const profileWallet = (): string => "/profile/wallet";
export const onboarding = (): string => "/onboarding";
export const lock = (): string => "/lock";

export const userProfile = (urn: string): string => `/user/${urn}`;
export const userProfileProjects = (urn: string): string =>
  `/user/${urn}/projects`;

export const projectIssues = (id: string): string => `/projects/${id}/issues`;
export const projectIssue = (id: string): string => `/projects/${id}/issue`;
export const projectRevisions = (id: string): string =>
  `/projects/${id}/revisions`;
export const projectUntracked = (urn: string): string =>
  `/projects/untracked/${urn}`;

export const projectSource = (projectId: string): string =>
  `/projects/${projectId}/source`;

export const parseQueryString = (querystring: string): ParsedQs => {
  return parse(querystring);
};

export const projectCommit = (id: string, hash: string): string =>
  `/projects/${id}/commit/${hash}`;
export const projectCommits = (id: string): string => `/projects/${id}/commits`;

export const designSystemGuide = (): string => "/design-system-guide";

// modal routes
export const newProject = (): string => "/new-project";
export const search = (): string => "/search";
export const shortcuts = (): string => "/shortcuts";

export const active = (
  path: string,
  location: string,
  loose = false
): boolean => {
  return regexparam(path, loose).pattern.test(location);
};
