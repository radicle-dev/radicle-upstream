export const mockChangeset = {
  summary: {
    additions: 32,
    deletions: 24,
  },
  files: [
    {
      path: "core/index.js",
      hunks: [
        {
          expanded: true,
          lines: [
            { num: [192, 192], type: "", content: "/*" },
            { num: [193, 193], type: "", content: " * Say hello" },
            { num: [194, 194], type: "", content: " */" },
            {
              num: [195, null],
              type: "-",
              content: "Server.prototype.hello = function (req, contentType) {",
            },
            {
              num: [null, 195],
              type: "+",
              content: "Server.prototype.hello = function (req, contentType) {",
            },
            {
              num: [196, 196],
              type: "",
              content: "    var enable = this.options.gzip;",
            },
            {
              num: [197, 197],
              type: "",
              content: "    if (enable && (typeof enable === 'boolean' ||",
            },
          ],
        },
      ],
    },
    {
      path: "core/server.js",
      hunks: [
        {
          expanded: true,
          lines: [
            {
              num: [192, 192],
              type: "",
              content:
                "/* Check if we should consider sending a gzip version of the file based on the",
            },
            {
              num: [193, 193],
              type: "",
              content:
                " * file content type and client's Accept-Encoding header value.",
            },
            { num: [194, 194], type: "", content: " */" },
            {
              num: [195, null],
              type: "-",
              content: "Server.prototype.ok = function (req, contentType) {",
            },
            {
              num: [null, 195],
              type: "+",
              content:
                "Server.prototype.gzipOk = function (req, contentType) {",
            },
            {
              num: [196, 196],
              type: "",
              content: "    var enable = this.options.gzip;",
            },
            { num: [197, 197], type: "", content: "    if (enable &&" },
            {
              num: [198, 198],
              type: "",
              content: "        (typeof enable === 'boolean' ||",
            },
          ],
        },
        {
          expanded: false,
          header:
            "@@ -206,20 +206,17 @@ Server.prototype.gzipOk = function(req, contentType) {",
          lines: [
            {
              num: [199, 199],
              type: "",
              content:
                "            (contentType && (enable instanceof RegExp) && enable.test(contentType)))) {",
            },
            {
              num: [200, 200],
              type: "",
              content:
                "        var acceptEncoding = req.headers['accept-encoding'];",
            },
            {
              num: [201, 201],
              type: "",
              content:
                "        return acceptEncoding && acceptEncoding.indexOf('gzip') >= 0;",
            },
            { num: [202, 202], type: "", content: "    }" },
            { num: [203, 203], type: "", content: "    return false;" },
            { num: [204, 204], type: "", content: "}" },
            { num: [205, 205], type: "", content: "" },
          ],
        },
        {
          expanded: true,
          lines: [
            {
              num: [206, null],
              type: "-",
              content:
                "Server.prototype.respond = function (pathname, status, contentType, _headers, files, stat, req, res, finish) {",
            },
            {
              num: [null, 206],
              type: "+",
              content:
                "/* Send a gzipped version of the file if the options and the client indicate gzip is enabled and",
            },
            {
              num: [null, 207],
              type: "+",
              content:
                " * we find a .gz file mathing the static resource requested.",
            },
            { num: [null, 208], type: "+", content: " */" },
            {
              num: [null, 209],
              type: "+",
              content:
                "Server.prototype.respondGzip = function (pathname, status, contentType, _headers, files, stat, req, res, finish) {",
            },
            {
              num: [207, 210],
              type: "",
              content: "    var that = this;",
            },
            {
              num: [208, 211],
              type: "",
              content:
                "    if (files.length == 1 && this.gzipOk(req, contentType)) {",
            },
            {
              num: [209, 212],
              type: "",
              content: "        var gzFile = files[0] + '.gz';",
            },
            {
              num: [210, 213],
              type: "",
              content: "        fs.stat(gzFile, function (e, gzStat) {",
            },
            {
              num: [211, 214],
              type: "",
              content: "            if (!e && gzStat.isFile()) {",
            },
            {
              num: [212, 215],
              type: "",
              content: "                var vary = _headers['Vary'];",
            },
            {
              num: [213, null],
              type: "-",
              content:
                "                _headers['Vary'] = (vary && vary != 'Accept-Encoding'?vary+', ':'')+'Accept-Encoding';",
            },
            {
              num: [null, 216],
              type: "+",
              content:
                "                _headers['Vary'] = (vary && vary != 'Accept-Encoding' ? vary + ', ' : '') + 'Accept-Encoding';",
            },
            {
              num: [214, 217],
              type: "",
              content: "                _headers['Content-Encoding'] = 'gzip';",
            },
            {
              num: [215, 218],
              type: "",
              content: "                stat.size = gzStat.size;",
            },
            {
              num: [216, 219],
              type: "",
              content: "                files = [gzFile];",
            },
            {
              num: [217, null],
              type: "-",
              content: "            } else {",
            },
            {
              num: [218, null],
              type: "-",
              content:
                "                console.log('gzip file not found or error finding it', gzFile, String(e), stat.isFile());",
            },
            { num: [219, 220], type: "", content: "            }" },
            {
              num: [220, 221],
              type: "",
              content:
                "            that.respondNoGzip(pathname, status, contentType, _headers, files, stat, req, res, finish);",
            },
            { num: [221, 222], type: "", content: "        });" },
          ],
        },
      ],
    },
  ],
};
