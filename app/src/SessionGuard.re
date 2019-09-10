open AppStore;
open Router;
open StoreSession;

[@react.component]
let make = (~children) => {
  let session = Store.useSelector(state => state.session);
  let dispatch = Store.useDispatch();

  switch (session) {
  | Present(_) => ()
  | NotPresent(_) =>
    StoreOverlay.Show((JoinNetwork, Projects))->OverlayAction |> dispatch
  };

  children;
};
