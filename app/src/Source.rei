module Project: {
  type project = {
    address: string,
    name: string,
    description: string,
    imgUrl: string,
  };

  let getProjects: unit => Js.Promise.t(array(project));
};
