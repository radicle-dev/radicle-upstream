open AppStore;
open Source;
open StoreMiddleware;
open StoreSession;

type dispatchFunc = thunk(appState) => unit;

let fetchSession = (dispatch: dispatchFunc, source: source) => {
  dispatch(SessionAction(Fetch));

  Js.Promise.(
    source.fetchAccount()
    |> then_(result =>
         switch (result) {
         | Belt.Result.Ok(account) =>
           SessionAction(Fetched(account)) |> dispatch |> resolve
         | Belt.Result.Error(reason) =>
           SessionAction(FetchFailed(reason)) |> dispatch |> resolve
         }
       )
  )
  |> ignore;
};
