type address = string;

type project = {
  address,
  name: string,
  description: string,
  imgUrl: string,
};

type fetchProjectsResult =
  | Success(array(project))
  | Error;

type source = {fetchProjects: unit => Js.Promise.t(fetchProjectsResult)};

let createMockSource: unit => source;
