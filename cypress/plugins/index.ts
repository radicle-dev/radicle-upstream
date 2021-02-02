import { nodeManagerPlugin } from "./nodeManager/plugin";

export default (
  on: Cypress.PluginEvents,
  _config: Cypress.PluginConfigOptions
): void => {
  on("task", { ...nodeManagerPlugin });
};
