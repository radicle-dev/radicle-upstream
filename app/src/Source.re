module Project = {
  type project = {
    address: string,
    name: string,
    description: string,
    imgUrl: string,
  };

  let mockProjects = [|
    {
      address: "monokel",
      name: "monokel",
      description: "A looking glass into the future.",
      imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
    },
  |];

  let getProjects = (): Js.Promise.t(array(project)) =>
    Js.Promise.make((~resolve, ~reject as _) =>
      Js.Global.setTimeout(() => resolve(. mockProjects), 1000) |> ignore
    );
};
