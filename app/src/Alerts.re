open AppStore;
open DesignSystem;
open Molecule;
open Alert;
open StoreAlerts;

[@react.component]
let make = () => {
  let dispatch = Store.useDispatch();
  let onClose = _ev => dispatch(AlertsAction(StoreAlerts.Remove));

  switch (Store.useSelector(state => state.alerts.latest)) {
  | Some(alert) =>
    <El style={margin(24, 0, 0, 0)}>
      <Alert onClick=onClose severity={alert.severity}>
        {React.string(alert.message)}
      </Alert>
    </El>
  | None => React.null
  };
};
