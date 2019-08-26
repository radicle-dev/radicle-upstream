type address = string;

type project = {
  address,
  name: string,
  description: string,
  imgUrl: string,
};

type fetchProjectsResult = Belt.Result.t(array(project), string);

type registerProjectResult = Belt.Result.t(project, string);

type source = {
  fetchProjects: unit => Js.Promise.t(fetchProjectsResult),
  registerProject:
    (~name: string, ~description: string, ~imgUrl: string) =>
    Js.Promise.t(registerProjectResult),
};

let createMockSource: unit => source;
