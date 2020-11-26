import * as lodash from "lodash";
import { ProxyProcessManager } from "./proxy-process-manager";

["stdout", "stderr"].forEach(output => {
  it(`collects and writes ${output}`, async () => {
    const redirection = output == "stderr" ? "1>&2" : "";
    const manager = new ProxyProcessManager(
      "sh",
      [
        "-c",
        `x=0; while [ $x -lt 300 ]; do echo "$x" ${redirection}; x=$(( x + 1 )); done`,
      ],
      true
    );

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

    expect(lines.length).toBe(200);

    expect(lines[0]).toBe("100");
    expect(lines.pop()).toBe("299");

    expect(processOutput).toEqual(`${lodash.range(0, 300).join("\n")}\n`);
  });
});

it("returns exit code", async () => {
  const manager = new ProxyProcessManager("sh", ["-c", "exit 50"], true);
  const result = await manager.run();
  expect(result.status).toBe(50);
});

it("returns the signal", async () => {
  const manager = new ProxyProcessManager("sleep", ["1"], true);
  const resultPromise = manager.run();
  manager.kill();
  const result = await resultPromise;
  expect(result.signal).toBe("SIGTERM");
});
