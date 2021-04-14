import lodash from "lodash";

export const throttled: (callback: () => void) => void = lodash.throttle(
  callback => {
    callback();
  },
  1000, // 1 second
  { trailing: false }
);

export const parseRadicleUrl = (url: string): undefined | string => {
  if (
    typeof url !== "string" ||
    url.length === 0 ||
    Buffer.byteLength(url, "utf8") > 1024 ||
    !url.toLowerCase().match(/^radicle:\/\//)
  ) {
    return;
  }

  return url;
};
