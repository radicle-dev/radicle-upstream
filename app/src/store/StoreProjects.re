type action =
  | Fetching
  | Fetched(array(Source.project))
  | FetchFailed(string)
  | Registered(Source.project)
  | RegisterFailed(string);

type state = {
  error: option(string),
  loading: bool,
  projects: option(array(Source.project)),
};
/* type state = */
/*   | Idle */
/*   | Loading */
/*   | Loaded(array(Source.project)) */
/*   | Errored; */

let initialState = {error: None, loading: false, projects: None};

let reducer = (state, action) =>
  switch (action) {
  | Fetching => {...state, loading: true}
  | Fetched(projects) => {
      ...state,
      loading: false,
      projects: Some(projects),
    }
  | FetchFailed(reason) => {...state, error: Some(reason)}
  | Registered(project) => {
      ...state,
      projects:
        Some(
          Belt.Option.getWithDefault(state.projects, [||])
          |> Array.append([|project|]),
        ),
    }
  | RegisterFailed(reason) => {...state, error: Some(reason)}
  };
/* switch ((state, action)) { */
/* | (Idle, Fetching) => Loading */
/* | (Loading, Fetched(projects)) => Loaded(projects) */
/* | (_, FetchFailed | RegisterFailed) => Errored */
/* | (Loaded(projects), Registered(project)) => Loaded(Array.append(projects, [| project |])) */
/* | (_, Registered(_project)) => state */
/* }; */
