import { get } from "svelte/store";

import * as navigation from "./navigation";

describe("navigation", () => {
  it("push", () => {
    expect(get(navigation.current)).toEqual({
      screen: navigation.screenBlank(),
    });

    navigation.push(navigation.screenOnboarding());

    expect(get(navigation.current)).toEqual({
      screen: navigation.screenOnboarding(),
    });
  });
});
