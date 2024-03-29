// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as lodash from "lodash";
import { ProxyProcessManager } from "./proxy-process-manager";

["stdout", "stderr"].forEach(output => {
  it(`collects and writes ${output}`, async () => {
    const redirection = output === "stderr" ? "1>&2" : "";
    const manager = new ProxyProcessManager({
      proxyPath: "sh",
      proxyArgs: [
        "-c",
        `x=0; while [ $x -lt 30 ]; do echo "$x" ${redirection}; x=$(( x + 1 )); done`,
      ],
      lineLimit: 20,
    });

    let processOutput = "";
    const outputStream = output === "stdout" ? process.stdout : process.stderr;
    const write = jest
      .spyOn(outputStream, "write")
      .mockImplementation(chunk => {
        processOutput += chunk;
        return true;
      });
    const result = await manager.run();
    write.mockRestore();

    expect(result.status).toBe(0);

    const lines = result.output.split("\n");

    expect(lines.length).toBe(20);

    expect(lines[0]).toBe("10");
    expect(lines.pop()).toBe("29");

    expect(processOutput).toEqual(`${lodash.range(0, 30).join("\n")}\n`);
  });
});

it("returns exit code", async () => {
  const manager = new ProxyProcessManager({
    proxyPath: "sh",
    proxyArgs: ["-c", "exit 50"],
    lineLimit: 1,
  });
  const result = await manager.run();
  expect(result.status).toBe(50);
});

it("returns the signal", async () => {
  const manager = new ProxyProcessManager({
    proxyPath: "sleep",
    proxyArgs: ["1"],
    lineLimit: 1,
  });
  const resultPromise = manager.run();
  const shutdownResult = await manager.shutdown().catch(e => e);
  expect(shutdownResult.signal).toBe("SIGTERM");
  const result = await resultPromise;
  expect(result.signal).toBe("SIGTERM");
});
