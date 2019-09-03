open Source;

type appState = {
  projectState: StoreProject.state,
  projectsState: StoreProjects.state,
  session: StoreSession.state,
};

type StoreMiddleware.thunk(_) +=
  | ProjectAction(StoreProject.action)
  | ProjectsAction(StoreProjects.action)
  | SessionAction(StoreSession.action);

let appReducer = (state: appState, action) =>
  switch (action) {
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
        projectState: StoreProject.initialState,
        projectsState: StoreProjects.initialState,
        session: StoreSession.initialState,
      },
      ~enhancer=thunkEnhancer,
      (),
    );

  store;
};
