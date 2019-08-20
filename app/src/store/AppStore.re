open Source.Project;

type projectsAction =
  | Fetch;

type projectsState =
  | Loading
  | Fetched(array(project));

let projectInitialState = Loading;

let projectsReducer = (_state, action) =>
  switch (action) {
  | Fetch => Fetched([||])
  };

type thunk('state) = ..;
type thunk('state) +=
  | Thunk((Reductive.Store.t(thunk('state), 'state) => unit));

type appState = {projects: projectsState};

type thunk(_) +=
  | ProjectsAction(projectsAction);

let appReducer = (state: appState, action) =>
  switch (action) {
  | ProjectsAction(action) => {
      projects: projectsReducer(state.projects, action),
    }
  | _ => state
  };

type t = Reductive.Store.t(thunk(appState), appState);

let createStore = (): t => {
  /* Enable support for redux dev tooling. */
  /* let storeEnhancer = */
  /*   ReductiveDevTools.( */
  /*     Connectors.reductiveEnhancer( */
  /*       Extension.enhancerOptions(~name="ReductiveApp", ()), */
  /*     ) */
  /*   ); */

  let store: t =
    Reductive.Store.create(
      ~reducer=appReducer,
      ~preloadedState={
        projects: projectInitialState,
      },
      ~enhancer=(_store, next) => next,
      (),
    );

  store;
};
