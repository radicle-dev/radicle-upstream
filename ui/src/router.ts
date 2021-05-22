import * as svelteStore from "svelte/store";

export type ProfileTab = "projects" | "following" | "funding";
export type ProjectTab = "files" | "commits" | "commit";
export type UserProfileTab = "projects";

export type Route =
  | { type: "empty" }
  | { type: "designSystemGuide" }
  | { type: "lock" }
  | { type: "onboarding" }
  | { type: "profile"; activeTab: ProfileTab }
  | { type: "userProfile"; activeTab: UserProfileTab; urn: string }
  | {
      type: "project";
      activeTab: ProjectTab;
      urn: string;
      commitHash: string | null;
    }
  | { type: "settings" };

const writableHistory: svelteStore.Writable<Route[]> = svelteStore.writable([]);

export const push = (newRoute: Route): void => {
  writableHistory.update(history => [...history, newRoute]);
};

export const pop = (): void => {
  writableHistory.update(history => history.slice(0, -1));
};

export const routeStore: svelteStore.Readable<Route> = svelteStore.derived(
  writableHistory,
  state => {
    if (state.length === 0) {
      return <Route>{ type: "empty" };
    } else {
      return state.slice(-1)[0];
    }
  }
);

routeStore.subscribe(state => {
  console.log(state);
});
