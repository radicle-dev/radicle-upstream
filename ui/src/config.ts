import type {} from "../../native/preload";
import qs from "qs";

export const HIDDEN_BRANCHES = ["rad/contributor", "rad/project"];
export const UPSTREAM_DEFAULT_BRANCH = "main";
export const GIT_DEFAULT_BRANCH = "master";
export const NOTIFICATION_TIMEOUT = 8000; // ms
export const FADE_DURATION = 200;

// `true` if we are running unit tests with Jest.
export const isNodeTestEnv = Boolean(
  globalThis.process && globalThis.process.env["NODE_ENV"] === "test"
);

// `true` if this code is run by the Cypress test driver.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const isCypressTestEnv = Boolean((globalThis as any).cy);

// `true` if the app is running in development mode or in test mode
export const isDev =
  isNodeTestEnv || (window.electron ? window.electron.isDev : true);

// `true` if experimental features should be enabled. This is
// controlled by the the `RADICLE_UPSTREAM_EXPERIMENTAL` flag.
//
// This is `true` in the cypress and node test environments.
export const isExperimental =
  isNodeTestEnv || (window.electron ? window.electron.isExperimental : false);

const query = qs.parse(
  isNodeTestEnv ? "" : window.location.search.replace("?", "")
);

// The address of the proxy in `host:port` format.
export const proxyAddress =
  typeof query.backend === "string" ? query.backend : "localhost:17246";
