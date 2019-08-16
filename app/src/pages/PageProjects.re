open Source.Project;
open Router;

module List = {
  [@react.component]
  let make = (~projects: array(project)) => {
    let ps =
      Array.map(
        project =>
          <li key={project.address}>
            <Link page={Project(project.address)}>
              {React.string(project.name)}
            </Link>
          </li>,
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
    <h1> {React.string("Explore projects")} </h1>
    {
      switch (state) {
      | Loading => <div> {React.string("Loading...")} </div>
      | Fetched(projects) => <List projects />
      | Failed(_error) => <div className="error" />
      }
    }
  </>;
};
