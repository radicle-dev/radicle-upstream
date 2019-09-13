open AppStore;
open DesignSystem;
open Molecule;
open StoreAlerts;
open Particle;

module Styles = {
  open Css;

  let alerts = style([backgroundColor(Color.white()), marginTop(px(24))]);
};

[@react.component]
let make = () => {
  let dispatch = Store.useDispatch();

  let alerts =
    Array.mapi(
      (index, alert) => {
        let onClose = _ev =>
          dispatch(AlertsAction(StoreAlerts.Remove(alert)));

        <Alert
          style={margin(0, 0, 8, 0)}
          onClick=onClose
          severity={alert.severity}
          key={index |> string_of_int}>
          {React.string(alert.message)}
        </Alert>;
      },
      Store.useSelector(state => state.alerts.all),
    )
    |> React.array;

  <El style=Styles.alerts> alerts </El>;
};
