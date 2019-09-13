open AppStore;
open StoreAlerts;
open Source;

let showAlert = (severity, message, dispatch, state, _source: source) => {
  let alert = {severity, message, id: state.alerts.idCounter};

  dispatch(AlertsAction(Add(alert)));

  Js.Global.setTimeout(() => dispatch(AlertsAction(Remove(alert))), 3000)
  |> ignore;
};
