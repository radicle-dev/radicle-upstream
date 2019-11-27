import { Icon } from "./DesignSystem";

import SearchPage from "./pages/Search.svelte";
import FeedPage from "./pages/Feed.svelte";
import ProjectsPage from "./pages/Projects.svelte";
import ProjectOverviewPage from "./pages/Project/Overview.svelte";
import ProjectFeedPage from "./pages/Project/Feed.svelte";
import ProjectMembersPage from "./pages/Project/Members.svelte";
import ProjectFundsPage from "./pages/Project/Funds.svelte";
import ProjectSourcePage from "./pages/Project/Source.svelte";
import ProjectCommitsPage from "./pages/Project/Commits.svelte";
import ProjectBranchesPage from "./pages/Project/Branches.svelte";
import DesignSystemPage from "./pages/DesignSystem.svelte";
import WalletPage from "./pages/Wallet.svelte";
import ProfilePage from "./pages/Profile.svelte";
import NotFoundPage from "./pages/NotFound.svelte";

const Search = {
  title: "Search",
  route: { "/search": SearchPage },
  path: () => "/search",
  icon: Icon.Search
};

const Feed = {
  title: "Feed",
  route: { "/feed": FeedPage },
  path: () => "/feed",
  icon: Icon.Feed
};

const Projects = {
  title: "Projects",
  route: { "/projects": ProjectsPage },
  path: () => "/projects",
  icon: Icon.Projects
};

const ProjectOverview = {
  title: "Overview",
  route: { "/projects/:domain/:name/overview": ProjectOverviewPage },
  path: id => `/projects/${id.domain}/${id.name}/overview`,
  icon: Icon.Home
};

const ProjectFeed = {
  title: "Feed",
  route: { "/projects/:domain/:name/feed": ProjectFeedPage },
  path: id => `/projects/${id.domain}/${id.name}/feed`,
  icon: Icon.Feed
};

const ProjectMembers = {
  title: "Members",
  route: { "/projects/:domain/:name/members": ProjectMembersPage },
  path: id => `/projects/${id.domain}/${id.name}/members`,
  icon: Icon.Member
};

const ProjectFunds = {
  title: "Funds",
  route: { "/projects/:domain/:name/funds": ProjectFundsPage },
  path: id => `/projects/${id.domain}/${id.name}/funds`,
  icon: Icon.Fund
};

const ProjectSource = {
  title: "Source",
  route: { "/projects/:domain/:name/source": ProjectSourcePage },
  path: id => `/projects/${id.domain}/${id.name}/source`,
  icon: Icon.Source
};

const ProjectCommits = {
  title: "Commits",
  route: { "/projects/:domain/:name/commits": ProjectCommitsPage },
  path: id => `/projects/${id.domain}/${id.name}/commits`,
  icon: Icon.Commit
};

const ProjectBranches = {
  title: "Branches",
  route: { "/projects/:domain/:name/branches": ProjectBranchesPage },
  path: id => `/projects/${id.domain}/${id.name}/branches`,
  icon: Icon.Branch
};

const DesignSystem = {
  title: "Design System",
  route: { "/design-system": DesignSystemPage },
  path: () => "/design-system",
  icon: Icon.Plus
};

const Wallet = {
  title: "Wallet",
  route: { "/wallet": WalletPage },
  path: () => "/wallet",
  icon: Icon.Fund
};

const Profile = {
  title: "Profile",
  route: { "/profile": ProfilePage },
  path: () => "/profile"
};

const NotFound = {
  route: { "*": NotFoundPage },
  path: () => ""
};

export const routes = {
  ...Search.route,
  ...Feed.route,
  ...Projects.route,
  ...ProjectOverview.route,
  ...ProjectFeed.route,
  ...ProjectMembers.route,
  ...ProjectFunds.route,
  ...ProjectSource.route,
  ...ProjectCommits.route,
  ...ProjectBranches.route,
  ...DesignSystem.route,
  ...Wallet.route,
  ...Profile.route,
  ...NotFound.route
};

export {
  Search,
  Feed,
  Projects,
  ProjectOverview,
  ProjectFeed,
  ProjectMembers,
  ProjectFunds,
  ProjectSource,
  ProjectCommits,
  ProjectBranches,
  DesignSystem,
  Wallet,
  Profile,
  NotFound
};
