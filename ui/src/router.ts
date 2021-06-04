import * as svelteStore from "svelte/store";

import type * as theGraphApi from "ui/src/theGraphApi";
import type * as project from "ui/src/project";

import * as error from "ui/src/error";
import * as org from "ui/src/org";
import * as screen from "ui/src/screen";

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

export type LoadedOrgTab =
  | {
      type: "projects";
      anchoredProjects: project.Project[];
      unresolvedAnchors: theGraphApi.ProjectAnchor[];
      gnosisSafeAddress: string;
    }
  | {
      type: "members";
      threshold: number;
      members: theGraphApi.Member[];
    };
export type LoadedRoute =
  | { type: "loading" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | {
      type: "org";
      address: string;
      gnosisSafeAddress: string;
      activeTab: LoadedOrgTab;
    }
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

// This is only respected by Safari.
const DOCUMENT_TITLE = "Radicle Upstream";

const DEFAULT_ROUTE: Route = { type: "profile", activeTab: "projects" };

const routeToPath = (route: Route | LoadedRoute): string => {
  let subRoute = "";

  if (route.type === "profile" || route.type === "networkDiagnostics") {
    subRoute = `/${route.activeTab}`;
  } else if (route.type === "project") {
    subRoute = `/${route.activeView.type}`;
  }

  return `#/${route.type}${subRoute}`;
};

const loadHistory = async (): Promise<void> => {
  if (window.history.state === null) {
    await push(DEFAULT_ROUTE);
    window.history.pushState(
      [DEFAULT_ROUTE],
      DOCUMENT_TITLE,
      routeToPath(DEFAULT_ROUTE)
    );
  } else {
    const persistedHistory: Route[] = window.history.state;
    const activeRoute = persistedHistory.slice(-1)[0];
    writableHistory.set(<LoadedRoute[]>persistedHistory.slice(0, -1));
    await push(activeRoute);
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

const writableHistory: svelteStore.Writable<LoadedRoute[]> =
  svelteStore.writable([{ type: "loading" }]);

export const push = async (newRoute: Route): Promise<void> => {
  let loadedRoute: LoadedRoute;

  switch (newRoute.type) {
    case "org":
      switch (newRoute.activeTab) {
        case "projects": {
          try {
            screen.lock();
            const orgScreenData = await org.fetchOrg(newRoute.address);
            const projectAnchorsData = await org.resolveProjectAnchors(
              newRoute.address
            );
            loadedRoute = {
              type: "org",
              address: newRoute.address,
              gnosisSafeAddress: orgScreenData.gnosisSafeAddress,
              activeTab: {
                type: "projects",
                anchoredProjects: projectAnchorsData.anchoredProjects,
                unresolvedAnchors: projectAnchorsData.unresolvedAnchors,
                gnosisSafeAddress: orgScreenData.gnosisSafeAddress,
              },
            };
          } finally {
            screen.unlock();
          }
          break;
        }
        case "members": {
          try {
            screen.lock();
            const orgScreenData = await org.fetchOrg(newRoute.address);
            const membersData = await org.fetchMembers(
              orgScreenData.gnosisSafeAddress
            );
            loadedRoute = {
              type: "org",
              address: newRoute.address,
              gnosisSafeAddress: orgScreenData.gnosisSafeAddress,
              activeTab: {
                type: "members",
                members: membersData.members,
                threshold: membersData.threshold,
              },
            };
          } finally {
            screen.unlock();
          }
          break;
        }
        default:
          loadedRoute = newRoute;
      }
      break;
    default:
      loadedRoute = newRoute;
  }

  // Limit history to a maximum of 10 steps. We shouldn't be doing more than
  // one subsequent pop() anyway.
  writableHistory.update(history => [...history, loadedRoute].slice(-10));
  persistHistory();
};

export const pop = (): void => {
  writableHistory.update(history => history.slice(0, -1));
  persistHistory();
};

export const activeRouteStore: svelteStore.Readable<LoadedRoute> =
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

export const initialize = (): void => {
  loadHistory();
};
