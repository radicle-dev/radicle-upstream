const { notarize } = require("electron-notarize");

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

  return await notarize({
    appBundleId: process.env.npm_package_build_appId,
    appPath: `${context.appOutDir}/${context.packager.appInfo.productFilename}.app`,
    appleId: process.env.APPLE_ID,
    appleIdPassword: process.env.APPLE_ID_PASSWORD,
  });
};
