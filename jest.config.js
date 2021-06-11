module.exports = {
  roots: ["<rootDir>/ui/src", "<rootDir>/native"],
  transform: {
    "^.+\\.ts$": "ts-jest",
  },
  testEnvironment: "jsdom",
  testRegex: "(/__tests__/.*|(\\.|/)(test))\\.ts$",
  moduleFileExtensions: ["ts", "js", "json"],
};
