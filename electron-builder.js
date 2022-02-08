// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

module.exports = {
  appId: "xyz.radicle.radicle-upstream",
  artifactName: "${name}.${ext}",
  afterSign: notarizeApp,
  files: [
    "public/**/*",
    "native/bundle.js",
    "native/bundle.js.map",
    "native/bundle.licenses.txt",
    "native/preload.js",
  ],
  directories: {
    buildResources: "public",
  },
  extraResources: [
    {
      from: "target/release/radicle-proxy",
      to: "./",
    },
    {
      from: "target/release/git-remote-rad",
      to: "./",
    },
    {
      from: "target/release/radicle-proxy.exe",
      to: "./",
    },
    {
      from: "target/release/git-remote-rad.exe",
      to: "./",
    },
    {
      from: "proxy/assets",
      to: "assets",
    },
  ],
  protocols: [
    {
      name: "radicle",
      schemes: ["radicle"],
    },
  ],
  linux: {
    target: ["Appimage"],
  },
  mac: {
    target: ["dmg"],
    hardenedRuntime: true,
    gatekeeperAssess: false,
    entitlements: "builder/entitlements.mac.plist",
    entitlementsInherit: "builder/entitlements.mac.plist",
    minimumSystemVersion: "10.14",
  },
};

async function notarizeApp(context) {
  if (process.env.NOTARIZE !== "true") {
    return;
  }

  if (context.electronPlatformName !== "darwin") {
    throw new Error("Notarization must be performad on macOS!");
  }

  if (
    !(
      process.env.APPLE_ID &&
      process.env.APPLE_ID_PASSWORD &&
      process.env.CSC_NAME
    )
  ) {
    throw new Error(
      "APPLE_ID, APPLE_ID_PASSWORD and CSC_NAME env variables must be set!"
    );
  }

  const { notarize } = await import("electron-notarize");

  await notarize({
    appBundleId: context.appId,
    appPath: `${context.appOutDir}/${context.packager.appInfo.productFilename}.app`,
    appleId: process.env.APPLE_ID,
    appleIdPassword: process.env.APPLE_ID_PASSWORD,
  });
}
