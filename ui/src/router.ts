import * as svelteStore from "svelte/store";
import * as error from "ui/src/error";

export type ProfileTab = "projects" | "following" | "funding";
export type ProjectTab = "files" | "commits" | "commit";

export type Route =
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "userProfile"; urn: string }
  | {
      type: "project";
      activeTab: ProjectTab;
      urn: string;
      commitHash: string | null;
    }
  | { type: "settings" };

const persistedState: Route | null = window.history.state;

const writableHistory: svelteStore.Writable<Route[]> = svelteStore.writable(
  persistedState === null ? [] : [persistedState]
);

const routeToPath = (route: Route): string => {
  let subRoute = "";

  if (route.type === "profile" || route.type === "project") {
    subRoute = `/${route.activeTab}`;
  }

  return `#/${route.type}${subRoute}`;
};

export const push = (newRoute: Route): void => {
  writableHistory.update(history => [...history, newRoute]);
  const title = "Radicle Upstream";
  window.history.pushState(newRoute, title, routeToPath(newRoute));
};

export const pop = (): void => {
  writableHistory.update(history => history.slice(0, -1));
  window.history.back();
};

export const activeRouteStore: svelteStore.Readable<Route> =
  svelteStore.derived(writableHistory, state => {
    if (state.length === 0) {
      return <Route>{ type: "profile", activeTab: "projects" };
    } else {
      return state.slice(-1)[0];
    }
  });

export const unreachable = (value: never): void => {
  throw new error.Error({
    code: error.Code.Unreachable,
    message: "Unreachable code",
    details: { value },
  });
};
