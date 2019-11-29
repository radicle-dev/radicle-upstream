import regexparam from "regexparam";

export const search = _params => "/search";
export const feed = _params => "/feed";
export const projects = _params => "/projects";

export const projectOverview = (domain, name) => `/projects/${domain}/${name}/`;
export const projectFeed = (domain, name) => `/projects/${domain}/${name}/feed`;
export const projectMembers = (domain, name) =>
  `/projects/${domain}/${name}/members`;
export const projectFunds = (domain, name) =>
  `/projects/${domain}/${name}/funds`;
export const projectSource = (domain, name, revision, objectType, path) => {
  if (revision && path) {
    return `/projects/${domain}/${name}/source/${revision}/${objectType}${path}`;
  } else {
    return `/projects/${domain}/${name}/source`;
  }
};

export const projectCommits = (domain, name) =>
  `/projects/${domain}/${name}/commits`;
export const projectBranches = (domain, name) =>
  `/projects/${domain}/${name}/branches`;

export const designSystem = _params => "/design-system";
export const wallet = _params => "/wallet";
export const profile = _params => "/profile";

export const active = (path, location, loose = false) => {
  return regexparam(path, loose).pattern.test(location);
};
