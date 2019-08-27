/** Unique identifier to refer to accounts of type proejct for now **/
type address = string;

/** The project representation on the registry. **/
type project = {
  address,
  name: string,
  description: string,
  imgUrl: string,
};

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
  fetchProjects: unit => Js.Promise.t(fetchProjectsResult),
  registerProject:
    (~name: string, ~description: string, ~imgUrl: string) =>
    Js.Promise.t(registerProjectResult),
};

/** Returns an implementatio of `source` which gives back local data. **/
let createLocalSource: unit => source;
