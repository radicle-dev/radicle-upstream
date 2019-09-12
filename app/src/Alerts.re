open AppStore;
open DesignSystem;
open Molecule;
open StoreAlerts;

[@react.component]
let make = () => {
  let dispatch = Store.useDispatch();

  let alerts = Store.useSelector(state => state.alerts.all);

  Array.mapi(
    (index, alert) => {
      let onClose = _ev =>
        dispatch(AlertsAction(StoreAlerts.Remove(alert)));

      <El style={margin(24, 0, 0, 0)} key={index |> string_of_int}>
        <Alert onClick=onClose severity={alert.severity}>
          {React.string(alert.message)}
        </Alert>
      </El>;
    },
    alerts,
  )
  |> React.array;
};
