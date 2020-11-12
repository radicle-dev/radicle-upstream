import { parse, ParsedQs } from "qs";
import regexparam from "regexparam";

import type { Urn } from "./urn";

export const blank = (): string => "/";
export const settings = (): string => "/settings";

export const profile = (): string => "/profile";
export const profileOnboard = (): string => "/profile/onboard";
export const profileProjects = (): string => "/profile/projects";
export const profileFollowing = (): string => "/profile/following";
export const profileWallet = (): string => "/profile/wallet";
export const onboarding = (): string => "/onboarding";
export const lock = (): string => "/lock";

export const userProfile = (urn: Urn): string => `/user/${urn}`;
export const userProfileProjects = (urn: Urn): string =>
  `/user/${urn}/projects`;

export const project = (urn: Urn): string => `/projects/${urn}`;
export const projectSourceCode = (urn: Urn): string =>
  `/projects/${urn}/source/code`;
export const projectSourceCommit = (urn: Urn, hash: string): string =>
  `/projects/${urn}/source/commit/${hash}`;
export const projectSourceCommits = (urn: Urn): string =>
  `/projects/${urn}/source/commits`;

export const parseQueryString = (querystring: string): ParsedQs => {
  return parse(querystring);
};

export const designSystemGuide = (): string => "/design-system-guide";

// modal routes
export const managePeers = (): string => "/manage-peers";
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
