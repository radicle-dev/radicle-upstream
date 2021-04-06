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

// `true` if the app is running in development mode
export const isDev = window.electron ? window.electron.isDev : true;

// Informs whether it's running in experimental mode, where
// features under construction are enabled and can thus be used.
export const isExperimental = window.electron
  ? window.electron.isExperimental
  : false;

const query = qs.parse(window.location.search.replace("?", ""));

// The address of the proxy in `host:port` format.
export const proxyAddress =
  typeof query.backend === "string" ? query.backend : "localhost:17246";
