module Project = {
  type project = {
    address: string,
    name: string,
    description: string,
    imgUrl: string,
  };

  let getProjects = (): array(project) => [|
    {
      address: "monokel",
      name: "monokel",
      description: "A looking glass into the future.",
      imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
    },
  |];
};
