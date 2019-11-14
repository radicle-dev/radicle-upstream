import { location } from "svelte-spa-router";
import { get } from "svelte/store";
import regexparam from "regexparam";

export const SEARCH = "/search";
export const FEED = "/feed";
export const PROJECTS = "/projects";

export const PROJECT_OVERVIEW = "/projects/:id/overview";
export const PROJECT_FEED = "/projects/:id/feed";
export const PROJECT_MEMBERS = "/projects/:id/members";
export const PROJECT_FUNDS = "/projects/:id/funds";
export const PROJECT_SOURCE = "/projects/:id/source";
export const PROJECT_COMMITS = "/projects/:id/commits";
export const PROJECT_BRANCHES = "/projects/:id/branches";

export const DESIGN_SYSTEM = "/design-system";
export const WALLET = "/wallet";
export const PROFILE = "/profile";
export const NOT_FOUND = "*";

// expand(PROJECT_OVERVIEW, {id: 123})
// > "/projects/123/overview"
export let makeHref = (path, params = {}) => {
  Object.keys(params).forEach(key => {
    path = path.replace(`:${key}`, params[key]);
  });

  return path;
};

// check if given path matches the current location
export const isActive = path => {
  return regexparam(path).pattern.test(get(location));
};
