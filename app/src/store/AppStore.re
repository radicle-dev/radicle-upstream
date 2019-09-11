open Source;

type appState = {
  overlay: StoreOverlay.state,
  projectState: StoreProject.state,
  projectsState: StoreProjects.state,
  session: StoreSession.state,
};

type StoreMiddleware.thunk(_) +=
  | OverlayAction(StoreOverlay.action)
  | ProjectAction(StoreProject.action)
  | ProjectsAction(StoreProjects.action)
  | SessionAction(StoreSession.action);

let appReducer = (state: appState, action) =>
  switch (action) {
  | OverlayAction(action) => {
      ...state,
      overlay: StoreOverlay.reducer(state.overlay, action),
    }
  | ProjectAction(action) => {
      ...state,
      projectState: StoreProject.reducer(state.projectState, action),
    }
  | ProjectsAction(action) => {
      ...state,
      projectsState: StoreProjects.reducer(state.projectsState, action),
    }
  | SessionAction(action) => {
      ...state,
      session: StoreSession.reducer(state.session, action),
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
        projectState: StoreProject.initialState,
        projectsState: StoreProjects.initialState,
        session: StoreSession.initialState,
      },
      ~enhancer=thunkEnhancer,
      (),
    );

  store;
};
