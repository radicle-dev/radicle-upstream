open Source;

type thunk('state) = ..;

type thunkFunc('state) = (thunk('state) => unit, 'state, source) => unit;

type thunk('state) +=
  | Thunk(thunkFunc('state));

let middleware = (source: source, store, next, action) =>
  switch (action) {
  | Thunk(func) =>
    func(
      Reductive.Store.dispatch(store),
      Reductive.Store.getState(store),
      source,
    )
  | _ => next(action)
  };

type appState = {projects: StoreProjects.state};

type thunk(_) +=
  | ProjectsAction(StoreProjects.action);

module ProjectsThunk = {
  let fetchProjects =
      (dispatch: thunk(appState) => unit, _state: appState, source: source) => {
    dispatch(ProjectsAction(StoreProjects.Fetching));

    Js.Promise.(
      source.fetchProjects()
      |> then_(result =>
           switch (result) {
           | Success(projects) =>
             ProjectsAction(StoreProjects.Fetched(projects))
             |> dispatch
             |> resolve
           | Error =>
             ProjectsAction(StoreProjects.FetchFailed) |> dispatch |> resolve
           }
         )
    )
    |> ignore;
  };
};

let appReducer = (state: appState, action) =>
  switch (action) {
  | ProjectsAction(action) => {
      projects: StoreProjects.reducer(state.projects, action),
    }
  | _ => state
  };

type t = Reductive.Store.t(thunk(appState), appState);

let createStore = (): t => {
  let thunkMiddleware = middleware(createMockSource());
  let thunkEnhancer = (store, next) => thunkMiddleware(store) @@ next;

  /* Enable support for redux dev tooling. */
  let storeEnhancer =
    ReductiveDevTools.(
      Connectors.reductiveEnhancer(
        Extension.enhancerOptions(~name="ReductiveApp", ()),
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
