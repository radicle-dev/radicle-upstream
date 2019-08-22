type action =
  | Fetching
  | Fetched(array(Source.project))
  | FetchFailed;

type state =
  | Idle
  | Loading
  | Loaded(array(Source.project))
  | Errored;

let initialState = Idle;

let reducer = (_state, action) =>
  switch (action) {
  | Fetching => Loading
  | Fetched(projects) => Loaded(projects)
  | FetchFailed => Errored
  };
