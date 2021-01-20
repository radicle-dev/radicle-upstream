import App from "./App.svelte";

const parseBackendAddress = (): string | undefined => {
  const search = window.location.search;

  if (search.includes("?backend=")) {
    const match = search.match(/\?backend=(.*)/);
    if (match) {
      return match[1];
    }
  }
};

const app = new App({
  target: document.body,
  props: { backendAddress: parseBackendAddress() },
});

export default app;
