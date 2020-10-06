declare module "web3-providers-ws" {
  // The type definitions in `web3-providers-ws` are incorrect.
  import { WebsocketProviderBase } from "web3-core-helpers";
  export default class WebsocketProvider extends WebsocketProviderBase {}
}
