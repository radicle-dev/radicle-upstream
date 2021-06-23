/* eslint-disable @typescript-eslint/no-var-requires */

// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

const { notarize } = require("electron-notarize");
const appId = require("../package.json").build.appId;

exports.default = async function notarizeApp(context) {
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

  if (!(typeof appId === "string" && appId.length > 0)) {
    throw new Error("build.appId must be set in package.json!");
  }

  return await notarize({
    appBundleId: appId,
    appPath: `${context.appOutDir}/${context.packager.appInfo.productFilename}.app`,
    appleId: process.env.APPLE_ID,
    appleIdPassword: process.env.APPLE_ID_PASSWORD,
  });
};
