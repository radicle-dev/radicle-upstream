type action =
  | Fetching
  | Fetched(Source.project)
  | FetchFailed(string);

type state =
  | Initial
  | Loading
  | Present(Source.project)
  | Failed(string);

let initialState = Initial;

let reducer = (_state, action) =>
  switch (action) {
  | Fetching => Loading
  | Fetched(project) => Present(project)
  | FetchFailed(reason) => Failed(reason)
  };
