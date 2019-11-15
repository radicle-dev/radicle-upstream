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

export const routes = {
  "/search": Search,
  "/feed": Feed,
  "/projects": Projects,

  "/projects/:id/overview": ProjectOverview,
  "/projects/:id/feed": ProjectFeed,
  "/projects/:id/members": ProjectMembers,
  "/projects/:id/funds": ProjectFunds,
  "/projects/:id/source": ProjectSource,
  "/projects/:id/commits": ProjectCommits,
  "/projects/:id/branches": ProjectBranches,

  "/design-system": DesignSystem,
  "/wallet": Wallet,
  "/profile": Profile,
  "*": NotFound
};
