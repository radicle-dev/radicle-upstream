// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import timezone from "dayjs/plugin/timezone"; // dependent on utc plugin
import relativeTime from "dayjs/plugin/relativeTime";

dayjs.extend(utc);
dayjs.extend(timezone);
dayjs.extend(relativeTime);

import { config } from "ui/src/config";

if (config.stubElectron) {
  import("ui/src/electronTestStub");
}

const { default: App } = await import("./App.svelte");

new App({
  target: document.body,
});
