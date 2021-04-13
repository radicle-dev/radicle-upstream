import type { Urn } from "./urn";

export const blank = (): string => "/";
export const settings = (): string => "/settings";

export const profile = (): string => "/profile";
export const profileOnboard = (): string => "/profile/onboard";
export const profileProjects = (): string => "/profile/projects";
export const profileFollowing = (): string => "/profile/following";
export const profileFunding = (): string => "/profile/funding";

export const onboarding = (): string => "/onboarding";
export const lock = (): string => "/lock";

export const userProfile = (urn: Urn): string => `/user/${urn}`;
export const userProfileProjects = (urn: Urn): string =>
  `/user/${urn}/projects`;

export const projectSourceFiles = (urn: Urn): string =>
  `/projects/${urn}/source/code`;
export const projectSourceCommit = (urn: Urn, hash: string): string =>
  `/projects/${urn}/source/commit/${hash}`;
export const projectSourceCommits = (urn: Urn): string =>
  `/projects/${urn}/source/commits`;
export const project = projectSourceFiles;

export const designSystemGuide = (): string => "/design-system-guide";

// modal routes
export const managePeers = (): string => "/manage-peers";
export const newProject = (): string => "/new-project";
export const search = (): string => "/search";
export const shortcuts = (): string => "/shortcuts";
export const walletQRCode = (): string => "/wallet/qrcode";
export const linkAddress = (): string => "/funding/link";
export const poolOnboarding = (): string => "/funding/pool/onboarding";
export const poolTopUp = (): string => "/funding/pool/top-up";
export const poolWithdraw = (): string => "/funding/pool/withdraw";
export const collectFunds = (): string => "/funding/pool/collect";
export const transaction = (): string => "/transaction";
