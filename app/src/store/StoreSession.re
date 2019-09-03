open Source;

type action =
  | Fetch
  | Fetched(option(account))
  | FetchFailed(string)
  | Created(account)
  | CreationFailed(string);

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
  | Created(account) => Present(account)
  | CreationFailed(reason) => Failed(reason)
  };
