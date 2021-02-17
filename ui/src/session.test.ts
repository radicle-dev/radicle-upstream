import { __test__ as session } from "./session";

describe("VALID_SEED_MATCH", () => {
  const peer_id = new Array(54).fill("a").join("");

  it("matches valid values", () => {
    const values = [
      `${peer_id}@radicle.xyz:1`,
      `${peer_id}@radicle.xyz:12345`,
      `${peer_id}@localhost:12345`,
      `${new Array(54).fill("1").join("")}@123.or:12345`,
      `${peer_id}@foo-bar.example.org:12345`,
    ];

    for (const value of values) {
      expect(value).toMatch(session.VALID_SEED_MATCH);
    }
  });

  it("does not match invalid values", () => {
    const values = [
      `radicle.xyz:12345`,
      `${peer_id}@radicle.xyz`,
      `${peer_id}@radicle?xyz:12345`,
    ];

    for (const value of values) {
      expect(value).not.toMatch(session.VALID_SEED_MATCH);
    }
  });
});
