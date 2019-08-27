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
  | Registered(_project) => initialState
  | RegisterFailed(reason) => {...state, error: Some(reason)}
  };
