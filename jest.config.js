module.exports = {
  roots: ["<rootDir>/ui/src", "<rootDir>/native"],
  transform: {
    "^.+\\.ts$": "ts-jest",
  },
  testRegex: "(/__tests__/.*|(\\.|/)(test))\\.ts$",
  moduleFileExtensions: ["ts", "js", "json"],
};
