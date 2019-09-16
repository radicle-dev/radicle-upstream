open AppStore;
open StoreAlerts;
open Source;

let showAlert = (severity, message, dispatch, _source: source) => {
  let alert = {severity, message, id: Random.bits()};

  dispatch(AlertsAction(Add(alert)));

  Js.Global.setTimeout(() => dispatch(AlertsAction(Remove(alert))), 3000)
  |> ignore;
};
