type appState = {projects: StoreProjects.state};

type thunk('state) = ..;

type thunkFunc('state) =
  (thunk('state) => unit, 'state, Source.source) => unit;

type thunk('state) +=
  | Thunk(thunkFunc('state));

type t = Reductive.Store.t(thunk(appState), appState);

let createStore: unit => t;

module ProjectsThunk: {
  let fetchProjects:
    (thunk(appState) => unit, appState, Source.source) => unit;
};
