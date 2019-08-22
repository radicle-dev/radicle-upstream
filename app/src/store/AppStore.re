open Source;

type projectsAction =
  | Fetching
  | Fetched(array(project))
  | FetchFailed;

type projectsState =
  | Idle
  | Loading
  | Loaded(array(project))
  | Errored;

let projectsInitialState = Idle;

let projectsReducer = (_state, action) =>
  switch (action) {
  | Fetching => Loading
  | Fetched(projects) => Loaded(projects)
  | FetchFailed => Errored
  };

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

type appState = {projects: projectsState};

type thunk(_) +=
  | ProjectsAction(projectsAction);

module ProjectsThunk = {
  let fetchProjects =
      (dispatch: thunk(appState) => unit, _state: appState, source: source) => {
    dispatch(ProjectsAction(Fetching));

    Js.Promise.(
      source.fetchProjects()
      |> then_(result =>
           switch (result) {
           | Success(projects) =>
             dispatch(ProjectsAction(Fetched(projects))) |> resolve
           | Error => dispatch(ProjectsAction(FetchFailed)) |> resolve
           }
         )
    )
    |> ignore;
  };
};

let appReducer = (state: appState, action) =>
  switch (action) {
  | ProjectsAction(action) => {
      projects: projectsReducer(state.projects, action),
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
      ~preloadedState={projects: projectsInitialState},
      ~enhancer=thunkEnhancer,
      (),
    );

  store;
};
