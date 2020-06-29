import qs from "qs";

interface Loc {
  location: string;
  querystring: string;
}

const getLocation = (): Loc => {
  const hashPosition = window.location.href.indexOf("#/");
  let location =
    hashPosition > -1 ? window.location.href.substr(hashPosition + 1) : "/";

  // Check if there's a querystring
  const qsPosition = location.indexOf("?");
  let querystring = "";
  if (qsPosition > -1) {
    querystring = location.substr(qsPosition + 1);
    location = location.substr(0, qsPosition);
  }
  return { location, querystring };
};

export enum Modal {
  Help = "/help",
  IdentityCreation = "/identity/new",
  OrgRegistration = "/orgs/register",
  MemberRegistration = "/orgs/:id/members/register",
  ProjectCreation = "/projects/new",
  ProjectRegistration = "/projects/register/:domainId",
  /* ProjectRegistration = "/projects/:projectId/register/:domainId", */
  UserRegistration = "/user-registration",
  TransactionDetails = "/transactions/:id",
}

export const modal = (loc: Loc, modal: Modal): string => {
  console.log(loc);
  const query = qs.parse(loc.querystring);
  query.modal = modal;

  return `${loc.location}?${qs.stringify(query)}`;
};
