open Source;

type appState = {projectsState: StoreProjects.state};

type StoreMiddleware.thunk(_) +=
  | ProjectsAction(StoreProjects.action);

let appReducer = (state: appState, action) =>
  switch (action) {
  | ProjectsAction(action) => {
      projectsState: StoreProjects.reducer(state.projectsState, action),
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
      ~preloadedState={projectsState: StoreProjects.initialState},
      ~enhancer=thunkEnhancer,
      (),
    );

  store;
};
