import { writable } from "svelte/store";

// TODO: define & structure this store

// e.g. does each value gets its own store...
//    const handle = "cloudhead" // as writable();
//    const shareableEntityIdentifier = cloudhead@ViJQHAdeZoiEbaE5vv83dpjEun.rad" // as writable();

// ...or is identity stored as an object with several values?
//    const identity = {
//      handle = "cloudhead";
//      shareableEntityIdentifier = "cloudhead@ViJQHAdeZoiEbaE5vv83dpjEun.rad"
//    } // as writable();

export const currentIdentityStore = writable(null);

export const setCurrentIdentity = identity =>
  currentIdentityStore.set(identity);
