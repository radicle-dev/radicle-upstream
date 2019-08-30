/** Unique identifier to refer to accounts of type proejct for now **/
type address = string;

type account = {
  avatarUrl: string,
  keyName: string,
};

type fetchAccountResult = Belt.Result.t(option(account), string);

/** The project representation on the registry. **/
type project = {
  address,
  name: string,
  description: string,
  imgUrl: string,
  members: array(account),
};

/** Returned for a project fetch, either the full project for the given address
 ** or a reason why the fetch operation failed.
 **/
type fetchProjectResult = Belt.Result.t(project, string);

/** Returned for fetched projects, either is the list of projects known to the
 ** ledger or a reason why the fetch operation failed.
 **/
type fetchProjectsResult = Belt.Result.t(array(project), string);

/** Returned for project registration, either the fully populated registered
 ** project or the reason why the operation failed.
 **/
type registerProjectResult = Belt.Result.t(project, string);

/** The type used to fetch data vital to the app. **/
type source = {
  fetchAccount: unit => Js.Promise.t(fetchAccountResult),
  fetchProject: address => Js.Promise.t(fetchProjectResult),
  fetchProjects: unit => Js.Promise.t(fetchProjectsResult),
  registerProject:
    (~name: string, ~description: string, ~imgUrl: string) =>
    Js.Promise.t(registerProjectResult),
};

/** Returns an implementation of `source` which gives back local data. **/
let createLocalSource: unit => source;
