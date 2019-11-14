import DesignSystem from "./pages/DesignSystem.svelte";
import Feed from "./pages/Feed.svelte";
import NotFound from "./pages/NotFound.svelte";
import Profile from "./pages/Profile.svelte";
import ProjectOverview from "./pages/Project/Overview.svelte";
import ProjectFeed from "./pages/Project/Feed.svelte";
import ProjectMembers from "./pages/Project/Members.svelte";
import ProjectFunds from "./pages/Project/Funds.svelte";
import ProjectSource from "./pages/Project/Source.svelte";
import ProjectCommits from "./pages/Project/Commits.svelte";
import ProjectBranches from "./pages/Project/Branches.svelte";
import Projects from "./pages/Projects.svelte";
import Search from "./pages/Search.svelte";
import Wallet from "./pages/Wallet.svelte";

import * as path from "./path.js";

export const routes = {
  [path.SEARCH]: Search,
  [path.FEED]: Feed,
  [path.PROJECTS]: Projects,

  [path.PROJECT_OVERVIEW]: ProjectOverview,
  [path.PROJECT_FEED]: ProjectFeed,
  [path.PROJECT_MEMBERS]: ProjectMembers,
  [path.PROJECT_FUNDS]: ProjectFunds,
  [path.PROJECT_SOURCE]: ProjectSource,
  [path.PROJECT_COMMITS]: ProjectCommits,
  [path.PROJECT_BRANCHES]: ProjectBranches,

  [path.DESIGN_SYSTEM]: DesignSystem,
  [path.WALLET]: Wallet,
  [path.PROFILE]: Profile,
  [path.NOT_FOUND]: NotFound
};
