import * as svelteStore from "svelte/store";
import * as error from "ui/src/error";

export type ProfileTab = "projects" | "following" | "funding";
export type ProjectView =
  | { type: "files" }
  | { type: "commits" }
  | { type: "commit"; commitHash: string }
  | { type: "patches"; filter: "open" | "closed" | "all" }
  | { type: "patch"; id: string; peerId: string };

export type NetworkDiagnosticsTab = "peers" | "requests";

export type Route =
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "networkDiagnostics"; activeTab: NetworkDiagnosticsTab }
  | { type: "userProfile"; urn: string }
  | {
      type: "project";
      urn: string;
      activeView: ProjectView;
    }
  | { type: "settings" };

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

let persistedState: Route = DEFAULT_ROUTE;

if (window.history.state === null) {
  window.history.pushState(
    DEFAULT_ROUTE,
    DOCUMENT_TITLE,
    routeToPath(DEFAULT_ROUTE)
  );
} else {
  persistedState = window.history.state;
}

const writableHistory: svelteStore.Writable<Route[]> = svelteStore.writable([
  persistedState,
]);

export const push = (newRoute: Route): void => {
  writableHistory.update(history => [...history, newRoute]);
  window.history.pushState(newRoute, DOCUMENT_TITLE, routeToPath(newRoute));
};

export const pop = (): void => {
  writableHistory.update(history => history.slice(0, -1));
  window.history.back();
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
