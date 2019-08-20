open Source.Project;

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

type fetchProjectsResult =
  | Success(array(project))
  | Error;

type projectsDataProvider = {
  fetchProjects: unit => Js.Promise.t(fetchProjectsResult),
};

let createProjectsDataProvider = () => {
  let mockProjects = [|
    {
      address: "monokel",
      name: "monokel",
      description: "A looking glass into the future.",
      imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1557488019/Frame_2_bhz6eq.svg",
    },
    {
      address: "monadic",
      name: "Monadic",
      description: "Open source organization of amazing things",
      imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
    },
    {
      address: "oscoin",
      name: "open source coin",
      description: "Infrastructure for the open source community",
      imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
    },
    {
      address: "radicle",
      name: "radicle",
      description: "Decentralized open source collaboration",
      imgUrl: "https://res.cloudinary.com/juliendonck/image/upload/v1549554598/monadic-icon_myhdjk.svg",
    },
  |];

  let fetchProjects = () =>
    Js.Promise.make((~resolve, ~reject as _) =>
      Js.Global.setTimeout(() => resolve(. Success(mockProjects)), 1000)
      |> ignore
    );
  /* Js.Promise.resolve(Success(mockProjects)); */

  {fetchProjects: fetchProjects};
};

type thunk('state) = ..;

type thunkFunc('state) =
  (thunk('state) => unit, 'state, projectsDataProvider) => unit;

type thunk('state) +=
  | Thunk(thunkFunc('state));

let middleware = (dataProvider: projectsDataProvider, store, next, action) =>
  switch (action) {
  | Thunk(func) =>
    func(
      Reductive.Store.dispatch(store),
      Reductive.Store.getState(store),
      dataProvider,
    )
  | _ => next(action)
  };

type appState = {projects: projectsState};

type thunk(_) +=
  | ProjectsAction(projectsAction);

module ProjectsThunk = {
  let fetchProjects =
      (
        dispatch: thunk(appState) => unit,
        _state: appState,
        dataProvider: projectsDataProvider,
      ) => {
    dispatch(ProjectsAction(Fetching));

    Js.Promise.(
      dataProvider.fetchProjects()
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
  let thunkMiddleware = middleware(createProjectsDataProvider());

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
