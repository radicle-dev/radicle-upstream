// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import * as svelteStore from "svelte/store";
import * as semver from "semver";
import * as router from "ui/src/router";
import * as browserStore from "ui/src/browserStore";

import * as ipc from "./ipc";
import * as modal from "./modal";
import * as notification from "./notification";
import * as session from "./session";
import { config } from "ui/src/config";

interface LatestVersionInfo {
  version: string;
  announcementUrl: string;
}

const fetchLatestVersion = async (): Promise<LatestVersionInfo> => {
  const response = await fetch("https://releases.radicle.xyz/latest.json");
  const body: LatestVersionInfo = await response.json();
  return body;
};

// Check for new version every 30 minutes. In testing mode, check every
// second.
const VERSION_CHECK_INTERVAL = config.e2eTest ? 1000 : 30 * 60 * 1000;

// Only notify about new version 14 days after the last notification.
// In testing mode, check 5 seconds.
const VERSION_NOTIFY_SILENCE_INTERVAL = config.e2eTest
  ? 5000
  : 14 * 24 * 60 * 60 * 1000;

const lastNotifiedStore = browserStore.create<number | null>(
  "radicle.settings.updateChecker.lastNotified",
  null,
  zod.number().nullable()
);

const isEnabledStore = browserStore.create<boolean | null>(
  "radicle.settings.updateChecker.isEnabled",
  null,
  zod.boolean().nullable()
);

class UpdateChecker {
  private checkInterval: number | null = null;

  private latestVersionInfo: svelteStore.Writable<
    LatestVersionInfo | undefined
  > = svelteStore.writable(undefined);

  private currentVersion: svelteStore.Writable<string | undefined> =
    svelteStore.writable(undefined);

  // Create an `UpdateChecker` and initialize it.
  //
  // If notifications have been enabled previously, we start checking
  // for updates.
  //
  // If the user has not configured update checking we ask them to do
  // so after onboarding.
  public static init(): UpdateChecker {
    const updateChecker = new UpdateChecker();
    if (svelteStore.get(isEnabledStore)) {
      updateChecker.enable();
    }

    session.waitUnsealed().then(() => {
      updateChecker.notifyEnable();
    });
    ipc.getVersion().then(currentVersion => {
      updateChecker.currentVersion.set(currentVersion);
    });

    return updateChecker;
  }

  // If the user has not been asked already, we show a notification
  // and ask them to enable update notifications.
  public notifyEnable(): void {
    const isEnabled = svelteStore.get(isEnabledStore);
    if (isEnabled === null) {
      notification.show({
        type: "primary",
        message: "Want to check for new versions automatically?",
        persist: true,
        actions: [
          {
            label: "Go to settings",
            handler: () => {
              modal.hide();
              router.push({ type: "settings" });
            },
          },
          {
            label: "Dismiss",
            handler: () => {},
          },
        ],
      });

      // After the user has been notified, we set this to the default
      isEnabledStore.set(false);
    }
  }

  // Enable background udpate checking.
  public enable(): void {
    isEnabledStore.set(true);

    this.checkNewVersion();
    if (this.checkInterval === null) {
      this.checkInterval = window.setInterval(() => {
        this.checkNewVersion();
      }, VERSION_CHECK_INTERVAL);
    }
  }

  // Disable background udpate checking.
  public disable(): void {
    isEnabledStore.set(false);

    if (this.checkInterval !== null) {
      clearInterval(this.checkInterval);
      this.checkInterval = null;
    }
  }

  // Returns if background update checking is enabled
  public isEnabled(): svelteStore.Readable<boolean> {
    return svelteStore.derived(isEnabledStore, isEnabled => {
      if (isEnabled === null) {
        return false;
      } else {
        return isEnabled;
      }
    });
  }

  // A store that holds the `LatestVersionInfo` if this fefature has
  // been enabled and if there is a newer version available.
  public newVersion(): svelteStore.Readable<LatestVersionInfo | undefined> {
    return svelteStore.derived(
      [this.latestVersionInfo, this.currentVersion],
      ([latestVersionInfo, currentVersion]) => {
        if (latestVersionInfo && currentVersion) {
          if (semver.gt(latestVersionInfo.version, currentVersion)) {
            return latestVersionInfo;
          } else {
            return undefined;
          }
        } else {
          return undefined;
        }
      }
    );
  }

  // Fetch information about the latest version. If that version is
  // newer than the current version and the user has not been notified
  // since `VERSION_NOTIFY_SILENCE_INTERVAL` we show a notification.
  private async checkNewVersion(): Promise<void> {
    let latestVersionInfo;
    try {
      latestVersionInfo = await fetchLatestVersion();
    } catch {
      return;
    }

    this.latestVersionInfo.set(latestVersionInfo);

    const { version, announcementUrl } = latestVersionInfo;

    const lastNotified = svelteStore.get(lastNotifiedStore);
    const now = Date.now();
    const hasBeenNotifiedRecently =
      lastNotified !== null &&
      now - lastNotified <= VERSION_NOTIFY_SILENCE_INTERVAL;

    const currentVersion = svelteStore.get(this.currentVersion);
    const isNewer =
      currentVersion !== undefined && semver.gt(version, currentVersion);

    if (!hasBeenNotifiedRecently && isNewer) {
      notification.show({
        type: "primary",
        message: "There is a new version of Upstream available",
        persist: true,
        actions: [
          {
            label: `Check out Version ${version}`,
            handler: () => {
              ipc.openUrl(announcementUrl);
            },
          },
          {
            label: "Dismiss",
            handler: () => {},
          },
        ],
      });
      lastNotifiedStore.set(now);
    }
  }
}

export const updateChecker = UpdateChecker.init();
