import * as svelteStore from "svelte/store";
import * as error from "ui/src/error";
import * as org from "ui/src/org";

export type NetworkDiagnosticsTab = "peers" | "requests";
export type OrgTab = "projects" | "members";
export type ProfileTab = "projects" | "following";
export type ProjectView =
  | { type: "files" }
  | { type: "commits" }
  | { type: "commit"; commitHash: string }
  | { type: "patches"; filter: "open" | "closed" | "all" }
  | { type: "patch"; id: string; peerId: string };
export type WalletTab = "transactions" | "tokenStreams";

export type Route =
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | { type: "org"; address: string; activeTab: OrgTab }
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | { type: "userProfile"; urn: string }
  | {
      type: "project";
      urn: string;
      activeView: ProjectView;
    }
  | { type: "wallet"; activeTab: WalletTab }
  | { type: "settings" };

const loadViewData = async (route: Route) => {
  switch (route.type) {
    case "org":
      if (route.activeTab === "projects") {
        await org.loadProjectsTabData(route.address);
      } else if (route.activeTab === "members") {
        await org.loadMembersTabData(route.address);
      }
      break;
    default:
    // NOOP
  }
};

// This is only respected by Safari.
const DOCUMENT_TITLE = "Radicle Upstream";

const DEFAULT_ROUTE: Route = { type: "profile", activeTab: "projects" };

const routeToPath = (route: Route): string => {
  let subRoute = "";

  if (route.type === "profile" || route.type === "networkDiagnostics") {
    subRoute = `/${route.activeTab}`;
  } else if (route.type === "project") {
    subRoute = `/${route.activeView.type}`;
  }

  return `#/${route.type}${subRoute}`;
};

const loadHistory = (): Route[] => {
  if (window.history.state === null) {
    loadViewData(DEFAULT_ROUTE);
    window.history.pushState(
      [DEFAULT_ROUTE],
      DOCUMENT_TITLE,
      routeToPath(DEFAULT_ROUTE)
    );
    return [DEFAULT_ROUTE];
  } else {
    const persistedHistory: Route[] = window.history.state;
    const activeRoute = persistedHistory.slice(-1)[0];
    loadViewData(activeRoute);

    return persistedHistory;
  }
};

const persistHistory = () => {
  const history = svelteStore.get(writableHistory);
  const activeRoute = svelteStore.get(activeRouteStore);
  // We're abusing the Electron History API here to work around a bug in
  // Electron which prevents us from navigating back after a browser refresh.
  window.history.replaceState(
    history,
    DOCUMENT_TITLE,
    routeToPath(activeRoute)
  );
};

// TODO: make this async so that loadViewData in loadHistory gets executed async, otherwise reloading a route with data breaks
const writableHistory: svelteStore.Writable<Route[]> = svelteStore.writable(
  loadHistory()
);

export const push = async (newRoute: Route): Promise<void> => {
  await loadViewData(newRoute);
  // Limit history to a maximum of 10 steps. We shouldn't be doing more than
  // one subsequent pop() anyway.
  writableHistory.update(history => [...history, newRoute].slice(-10));
  persistHistory();
};

// TODO: loadViewData here too
export const pop = (): void => {
  writableHistory.update(history => history.slice(0, -1));
  persistHistory();
};

export const activeRouteStore: svelteStore.Readable<Route> =
  svelteStore.derived(writableHistory, state => {
    return state.slice(-1)[0];
  });

export const unreachable = (value: never): void => {
  throw new error.Error({
    code: error.Code.Unreachable,
    message: "Unreachable code",
    details: { value },
  });
};
