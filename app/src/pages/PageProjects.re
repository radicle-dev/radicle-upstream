open AppStore;
open Atom;
open Router;
open Source.Project;

module List = {
  [@react.component]
  let make = (~projects: array(project)) => {
    let ps =
      Array.map(
        project =>
          <li key={project.address}>
            <Link page={Project(project.address)}>
              <Title> {React.string(project.name)} </Title>
              <p> {React.string(project.description)} </p>
              <img src={project.imgUrl} />
            </Link>
          </li>,
        projects,
      );

    <ul> {React.array(ps)} </ul>;
  };
};

[@react.component]
let make = () => {
  let state = Store.useSelector(state => state.projects);
  let dispatch = Store.useDispatch();

  if (state == Idle) {
    dispatch(Thunk(ProjectsThunk.fetchProjects)) |> ignore;
  };

  <>
    <div>
      <Title.Huge> {React.string("Explore")} </Title.Huge>
      <Button> {React.string("Register project")} </Button>
    </div>
    {
      switch (state) {
      | Idle
      | Loading => <div> {React.string("Loading...")} </div>
      | Loaded(projects) => <List projects />
      | Errored => <div className="error" />
      }
    }
  </>;
};
