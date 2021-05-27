import * as zod from "zod";
import * as router from "svelte-spa-router";
import * as svelteStore from "svelte/store";
import qs from "qs";

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

export enum PatchFilter {
  Open = "open",
  Closed = "closed",
  All = "all",
}

export interface PatchesQuery {
  filter: PatchFilter;
}

const patchesQuerySchema = zod
  .object({
    filter: zod.enum([PatchFilter.Open, PatchFilter.Closed, PatchFilter.All]),
  })
  .partial();

export const projectSourcePatchesFilter = (
  urn: Urn,
  filter: PatchFilter
): string => `${projectSourcePatches(urn)}?${qs.stringify({ filter })}`;

export const projectSourcePatches = (urn: Urn): string =>
  `/projects/${urn}/source/patches`;

// Holds the query paramters for the patches route if that route is
// active. Otherwise holds `undefined`
export const projectSourcePatchesQuery = (): svelteStore.Readable<
  PatchesQuery | undefined
> => {
  return svelteStore.derived(router.querystring, query => {
    const result = patchesQuerySchema.safeParse(qs.parse(query || ""));
    if (result.success) {
      const defaults = {
        filter: PatchFilter.Open,
      };
      return {
        ...defaults,
        ...result.data,
      };
    } else {
      return undefined;
    }
  });
};

export const projectSourcePatch = (
  projectUrn: Urn,
  peerId: string,
  id: string
): string =>
  `/projects/${projectUrn}/source/patch/${peerId}/${encodeURIComponent(id)}`;

export const project = projectSourceFiles;

export const designSystemGuide = (): string => "/design-system-guide";

export const networkDiagnosticsConnectedPeers = (): string =>
  "/network-diagnostics/connected-peers";
export const networkDiagnosticsWaitingRoom = (): string =>
  "/network-diagnostics/waiting-room";
