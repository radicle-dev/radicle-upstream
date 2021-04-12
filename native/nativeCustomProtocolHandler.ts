import lodash from "lodash";

const THROTTLE_TIMEOUT = 1000; // 1 second

export const handleCustomProtocolInvocation: (
  url: string,
  callback: (url: string) => void
) => void = lodash.throttle(
  (url, callback) => {
    if (
      typeof url !== "string" ||
      url.length === 0 ||
      Buffer.byteLength(url, "utf8") > 1024 ||
      !url.toLowerCase().match(/^radicle:\/\//)
    ) {
      return;
    }

    callback(url);
  },
  THROTTLE_TIMEOUT,
  { trailing: false }
);
