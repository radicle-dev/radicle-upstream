type address = string;

type account = {
  avatarUrl: string,
  keyName: string,
};

type fetchAccountResult = Belt.Result.t(option(account), string);

type project = {
  address,
  name: string,
  description: string,
  imgUrl: string,
};

type fetchProjectResult = Belt.Result.t(project, string);
type fetchProjectsResult = Belt.Result.t(array(project), string);
type registerProjectResult = Belt.Result.t(project, string);

type source = {
  fetchAccount: unit => Js.Promise.t(fetchAccountResult),
  fetchProject: address => Js.Promise.t(fetchProjectResult),
  fetchProjects: unit => Js.Promise.t(fetchProjectsResult),
  registerProject:
    (~name: string, ~description: string, ~imgUrl: string) =>
    Js.Promise.t(registerProjectResult),
};

let createLocalSource = () => {
  let localAccount = ref(None);
  let localProjects =
    ref([|
      {
        address: "monokel",
        name: "monokel",
        description: "A looking glass into the future.",
        imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
      },
      {
        address: "monadic",
        name: "Monadic",
        description: "Open source organization of amazing things",
        imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
      },
      {
        address: "oscoin",
        name: "open source coin",
        description: "Infrastructure for the open source community",
        imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
      },
      {
        address: "radicle",
        name: "radicle",
        description: "Decentralized open source collaboration",
        imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
      },
    |]);

  let fetchAccount = () =>
    Js.Promise.make((~resolve, ~reject as _) =>
      resolve(. Belt.Result.Ok(localAccount^))
    );

  let fetchProject = addr =>
    Js.Promise.make((~resolve, ~reject as _) => {
      let maybeProj = Belt.Array.getBy(localProjects^, p => addr == p.address);

      switch (maybeProj) {
      | Some(project) => resolve(. Belt.Result.Ok(project))
      | None => resolve(. Belt.Result.Error("Not Found"))
      };
    });

  let fetchProjects = () =>
    Js.Promise.make((~resolve, ~reject as _) =>
      Js.Global.setTimeout(
        () => resolve(. Belt.Result.Ok(localProjects^)),
        1000,
      )
      |> ignore
    );

  let registerProject = (~name: string, ~description: string, ~imgUrl: string) =>
    Js.Promise.make((~resolve, ~reject as _) => {
      let project = {address: "", name, description, imgUrl};

      localProjects := Array.append(localProjects^, [|project|]);

      resolve(. Belt.Result.Ok(project));
    });

  {fetchAccount, fetchProject, fetchProjects, registerProject};
};
