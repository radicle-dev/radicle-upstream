open AppStore;
open Source;
open StoreProjects;

let fetchProjects =
    (
      dispatch: StoreMiddleware.thunk(appState) => unit,
      _state: appState,
      source: source,
    ) => {
  dispatch(ProjectsAction(Fetching));

  Js.Promise.(
    source.fetchProjects()
    |> then_(result =>
         switch (result) {
         | Belt.Result.Ok(projects) =>
           ProjectsAction(Fetched(projects)) |> dispatch |> resolve
         | Belt.Result.Error(reason) =>
           ProjectsAction(FetchFailed(reason)) |> dispatch |> resolve
         }
       )
  )
  |> ignore;
};

let registerProject =
    (
      name: string,
      description: string,
      imgUrl: string,
      dispatch: StoreMiddleware.thunk(appState) => unit,
      _state: appState,
      source: source,
    ) =>
  Js.Promise.(
    source.registerProject(~name, ~description, ~imgUrl)
    |> then_(result =>
         switch (result) {
         | Belt.Result.Ok(project) =>
           Router.navigateOfPage(Router.Projects, ());
           ProjectsAction(Registered(project)) |> dispatch |> resolve;
         | Belt.Result.Error(reason) =>
           ProjectsAction(RegisterFailed(reason)) |> dispatch |> resolve
         }
       )
  )
  |> ignore;
