module Project: {
  type project = {
    address: string,
    name: string,
    description: string,
    imgUrl: string,
  };

  let getProjects: unit => array(project);
};
