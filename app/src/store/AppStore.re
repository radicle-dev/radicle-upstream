open Source;

type appState = {projects: StoreProjects.state};

type StoreMiddleware.thunk(_) +=
  | ProjectsAction(StoreProjects.action);

let appReducer = (state: appState, action) =>
  switch (action) {
  | ProjectsAction(action) => {
      projects: StoreProjects.reducer(state.projects, action),
    }
  | _ => state
  };

type t = Reductive.Store.t(StoreMiddleware.thunk(appState), appState);

let createStore = (): t => {
  let thunkMiddleware = StoreMiddleware.middleware(createMockSource());
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
      ~preloadedState={projects: StoreProjects.initialState},
      ~enhancer=thunkEnhancer,
      (),
    );

  store;
};
