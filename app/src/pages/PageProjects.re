open AppStore;
open Atom;
open Atom.Layout;
open Molecule;
open Source.Project;

module Styles = {
  open Css;

  let projectHeading = style([marginBottom(px(48)), marginTop(px(94))]);
};

module List = {
  [@react.component]
  let make = (~projects: array(project)) => {
    let ps =
      Array.map(
        project =>
          <Link page={Router.Project(project.address)}>
            <ProjectCard
              imgUrl={project.imgUrl}
              name={project.name}
              description={project.description}
            />
          </Link>,
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
    <div className=Styles.projectHeading>
      <Container.TwoColumns>
        ...(
             <Title.Huge> {React.string("Explore")} </Title.Huge>,
             <Button> {React.string("Register project")} </Button>,
           )
      </Container.TwoColumns>
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
