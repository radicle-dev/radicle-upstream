type alert = {
  severity: Molecule.Alert.severity,
  message: string,
};

type action =
  | Show(alert)
  | Remove;

type state = {latest: option(alert)};

let initialState = {latest: None};

let reducer = (_state, action) =>
  switch (action) {
  | Show(alert) => {latest: Some(alert)}
  | Remove => {latest: None}
  };
