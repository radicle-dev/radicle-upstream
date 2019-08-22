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

let createMockSource = () => {
  let mockProjects = [|
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
  |];

  let fetchProjects = () =>
    Js.Promise.make((~resolve, ~reject as _) =>
      Js.Global.setTimeout(() => resolve(. Success(mockProjects)), 1000)
      |> ignore
    );

  {fetchProjects: fetchProjects};
};
