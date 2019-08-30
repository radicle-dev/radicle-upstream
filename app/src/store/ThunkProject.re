open AppStore;
open Source;
open StoreMiddleware;
open StoreProject;

let fetchProject = (dispatch: thunk(appState) => unit, _source: source) =>
  dispatch(ProjectAction(Fetching));
