open Source.Project;
open Atom;
open Layout;
open Molecule;

module Styles = {
  open Css;

  let item =
    style([
      margin(px(16)),
      borderBottom(px(1), solid, gray),
      lastChild([borderWidth(px(0))]),
    ]);
};

module List = {
  [@react.component]
  let make = (~projects: array(project)) => {
    let ps = Array.map(project => <ProjectCard project />, projects);

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
    <Container.TwoColumns>
      ...(
           <Title.Huge> {React.string("Explore")} </Title.Huge>,
           <Button> {React.string("Register project")} </Button>,
         )
    </Container.TwoColumns>
    {
      switch (state) {
      | Loading => <div> {React.string("Loading...")} </div>
      | Fetched(projects) => <List projects />
      | Failed(_error) => <div className="error" />
      }
    }
  </>;
};
