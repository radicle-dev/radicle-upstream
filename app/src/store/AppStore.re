open Source;

type appState = {
  overlay: StoreOverlay.state,
  session: StoreSession.state,
  alerts: StoreAlerts.state,
};

type StoreMiddleware.thunk(_) +=
  | OverlayAction(StoreOverlay.action)
  | SessionAction(StoreSession.action)
  | AlertsAction(StoreAlerts.action);

let appReducer = (state: appState, action) =>
  switch (action) {
  | OverlayAction(action) => {
      ...state,
      overlay: StoreOverlay.reducer(state.overlay, action),
    }
  | SessionAction(action) => {
      ...state,
      session: StoreSession.reducer(state.session, action),
    }
  | AlertsAction(action) => {
      ...state,
      alerts: StoreAlerts.reducer(state.alerts, action),
    }
  | _ => state
  };

type t = Reductive.Store.t(StoreMiddleware.thunk(appState), appState);

let createStore = (): t => {
  let thunkMiddleware = StoreMiddleware.middleware(createLocalSource());
  let thunkEnhancer = (store, next) => thunkMiddleware(store) @@ next;

  /* Enable support for redux dev tooling. */
  let storeEnhancer =
    ReductiveDevTools.(
      Connectors.reductiveEnhancer(
        Extension.enhancerOptions(~name="oscoin.AppStore", ()),
      )
    );

  let store: t =
    (storeEnhancer @@ Reductive.Store.create)(
      ~reducer=appReducer,
      ~preloadedState={
        overlay: StoreOverlay.initialState,
        session: StoreSession.initialState,
        alerts: StoreAlerts.initialState,
      },
      ~enhancer=thunkEnhancer,
      (),
    );

  store;
};
