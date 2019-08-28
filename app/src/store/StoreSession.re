type action =
  | Fetch
  | Fetched(option(Source.account))
  | FetchFailed(string);

type state =
  | Initial
  | Empty
  | Fetching
  | Present(Source.account)
  | Failed(string);

let initialState = Initial;

let reducer = (_state, action) =>
  switch (action) {
  | Fetch => Fetching
  | Fetched(maybeAccount) =>
    switch (maybeAccount) {
    | Some(account) => Present(account)
    | None => Empty
    }
  | FetchFailed(reason) => Failed(reason)
  };
