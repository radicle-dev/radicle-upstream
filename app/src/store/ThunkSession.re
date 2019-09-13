open AppStore;
open Molecule;
open Source;
open StoreAlerts;
open StoreMiddleware;
open StoreSession;

type dispatchFunc = thunk(appState) => unit;

let fetchSession = (dispatch: dispatchFunc, source: source) => {
  dispatch(SessionAction(Fetch));

  Js.Promise.(
    source.fetchAccount()
    |> then_(result =>
         switch (result) {
         | Belt.Result.Ok(maybeAccount) =>
           SessionAction(Fetched(maybeAccount)) |> dispatch |> resolve
         | Belt.Result.Error(reason) =>
           SessionAction(FetchFailed(reason)) |> dispatch |> resolve
         }
       )
  )
  |> ignore;
};

let createAccount =
    (
      keyName: string,
      avatarUrl: string,
      next: Router.page,
      dispatch: dispatchFunc,
      source: source,
    ) => {
  dispatch(SessionAction(Fetch));

  Js.Promise.(
    source.createAccount(keyName, avatarUrl)
    |> then_(result =>
         switch (result) {
         | Belt.Result.Ok(account) =>
           Router.navigateToPage(next, ());
           SessionAction(Created(account)) |> dispatch;
           AlertsAction(
             Show({
               severity: Alert.Success,
               message: "Welcome " ++ keyName,
               id: 0,
             }),
           )
           |> dispatch
           |> resolve;
         | Belt.Result.Error(reason) =>
           SessionAction(CreationFailed(reason)) |> dispatch |> resolve
         }
       )
  )
  |> ignore;
};
