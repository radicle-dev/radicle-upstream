open AppStore;
open Router;
open StoreSession;

[@react.component]
let make = (~children) => {
  let dispatch = Store.useDispatch();
  let overlay = Store.useSelector(state => state.overlay);
  let session = Store.useSelector(state => state.session);

  switch (session) {
  | Present(_) =>
    switch (overlay) {
    | None => ()
    | Some(_) => StoreOverlay.Hide->OverlayAction |> dispatch
    }
  | NotPresent(_) =>
    StoreOverlay.Show((JoinNetwork, Projects))->OverlayAction |> dispatch
  };

  children;
};
