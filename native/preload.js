const childProcess = require("child_process");
const ipcRenderer = require("electron").ipcRenderer;

window.electron = {
  ipcRenderer: {
    invoke: ipcRenderer.invoke.bind(ipcRenderer),
    on: ipcRenderer.on.bind(ipcRenderer),
  },
  isDev: process.env.NODE_ENV === "development",
  isExperimental: process.env.RADICLE_UPSTREAM_EXPERIMENTAL === "true",
};

// Retrieve the user-defined global git default branch.
// It fails when the git version of the machine we are running
// on is lower than v2.28, retuning 'undefined' in such case.
window.usersGitDefaultBranch = () => {
  try {
    return childProcess
      .execSync("git config --global --get init.defaultBranch", {
        encoding: "utf-8",
      })
      .trim();
  } catch {
    return undefined;
  }
};
