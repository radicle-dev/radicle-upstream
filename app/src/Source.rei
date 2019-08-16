module Project: {
  type address = string;

  type project = {
    address,
    name: string,
    description: string,
    imgUrl: string,
  };

  let getProjects: unit => Js.Promise.t(array(project));
  let registerProject: address => unit;
};
