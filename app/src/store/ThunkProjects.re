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
         | Success(projects) =>
           ProjectsAction(Fetched(projects)) |> dispatch |> resolve
         | Error => ProjectsAction(FetchFailed) |> dispatch |> resolve
         }
       )
  )
  |> ignore;
};
