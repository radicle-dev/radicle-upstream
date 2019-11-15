import regexparam from "regexparam";

export const search = _params => "/search";
export const feed = _params => "/feed";
export const projects = _params => "/projects";

export const projectOverview = (params = {}) =>
  `/projects/${params.id}/overview`;
export const projectFeed = (params = {}) => `/projects/${params.id}/feed`;
export const projectMembers = (params = {}) => `/projects/${params.id}/members`;
export const projectFunds = (params = {}) => `/projects/${params.id}/funds`;
export const projectSource = (params = {}) => `/projects/${params.id}/source`;

export const projectCommits = (params = {}) => `/projects/${params.id}/commits`;
export const projectBranches = (params = {}) =>
  `/projects/${params.id}/branches`;

export const designSystem = _params => "/design-system";
export const wallet = _params => "/wallet";
export const profile = _params => "/profile";

export const active = (path, location) => {
  return regexparam(path).pattern.test(location);
};
