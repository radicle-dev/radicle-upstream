module.exports = {
  report: "detailed",
  production: "true",
  exclude: [
    // twemojji is licensed under MIT and CC-BY-4.0 but uses a
    // non-standard `license` field so that it cannot be parse
    // properly. https://github.com/twitter/twemoji/pull/499
    "twemoji",
  ],
  // Must only include licenses that are GPLv3 compatible. This is mostly
  // sourced from http://www.gnu.org/licenses/license-list.html
  allow: [
    // 0BSD is less restrictive than ISC https://opensource.org/licenses/0BSD
    "0BSD",
    // http://www.gnu.org/licenses/license-list.html#apache2
    "Apache-2.0",
    // http://www.gnu.org/licenses/license-list.html#FreeBSD
    "BSD-2-Clause",
    // http://www.gnu.org/licenses/license-list.html#ModifiedBSD
    "BSD-3-Clause",
    // http://www.gnu.org/licenses/license-list.html#ccby
    "CC-BY-3.0",
    "CC-BY-4.0",
    // http://www.gnu.org/licenses/license-list.html#CC0
    "CC0-1.0",
    "GPL-3.0-only",
    // http://www.gnu.org/licenses/license-list.html#ISC
    "ISC",
    // http://www.gnu.org/licenses/license-list.html#LGPLv3
    "LGPL-3.0",
    // Named "Expat" on the GNU license overview
    // http://www.gnu.org/licenses/license-list.html#Expat
    "MIT",
    // http://www.gnu.org/licenses/license-list.html#MPL-2.0
    "MPL-2.0",
    // http://www.gnu.org/licenses/license-list.html#Unlicense
    "Unlicense",
    // http://www.gnu.org/licenses/license-list.html#WTFPL
    "WTFPL",
    // http://www.gnu.org/licenses/license-list.html#ZLib
    "Zlib",
  ],
};
