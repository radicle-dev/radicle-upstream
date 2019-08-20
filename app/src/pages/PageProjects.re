open Source.Project;
open Atom;
open Layout;
open Molecule;

module Styles = {
  open Css;

  let projectHeading = style([marginTop(px(94)), marginBottom(px(48))]);
  let link = style([display(`flex)]);
};

module List = {
  [@react.component]
  let make = (~projects: array(project)) => {
    let ps =
      Array.map(
        project =>
          <Link style=Styles.link page={Router.Project(project.address)}>
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

type action =
  | ProjectsFetched(array(project));

type state =
  | Loading
  | Fetched(array(project))
  | Failed(string);

[@react.component]
let make = () => {
  let (state, dispatch) =
    React.useReducer(
      (_state, action) =>
        switch (action) {
        | ProjectsFetched(ps) => Fetched(ps)
        },
      Loading,
    );

  React.useEffect0(() => {
    getProjects()
    |> Js.Promise.then_(projects =>
         ProjectsFetched(projects) |> dispatch |> Js.Promise.resolve
       )
    |> ignore;

    None;
  });

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
      | Loading => <div> {React.string("Loading...")} </div>
      | Fetched(projects) => <List projects />
      | Failed(_error) => <div className="error" />
      }
    }
  </>;
};
