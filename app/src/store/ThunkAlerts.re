open AppStore;
open StoreAlerts;
open Source;

let showAlert = (severity, message, dispatch, state, _source: source) => {
  let alert = {severity, message, id: state.alerts.idCounter};

  dispatch(AlertsAction(Add(alert)));

  Js.Promise.make((~resolve, ~reject as _) =>
    Js.Global.setTimeout(
      () =>
        resolve(. Belt.Result.Ok(dispatch(AlertsAction(Remove(alert))))),
      3000,
    )
    |> ignore
  )
  |> ignore;
};
