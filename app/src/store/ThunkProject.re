open AppStore;
open Source;
open StoreMiddleware;
open StoreProject;

let fetchProject =
    (address: string, dispatch: thunk(appState) => unit, source: source) => {
  dispatch(ProjectAction(Fetching));

  Js.Promise.(
    source.fetchProject(address)
    |> then_(result =>
         switch (result) {
         | Belt.Result.Ok(project) =>
           ProjectAction(Fetched(project)) |> dispatch |> resolve
         | Belt.Result.Error(reason) =>
           ProjectAction(FetchFailed(reason)) |> dispatch |> resolve
         }
       )
  )
  |> ignore;
};
