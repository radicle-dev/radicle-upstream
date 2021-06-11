module.exports = {
  roots: ["<rootDir>/ui/src", "<rootDir>/native"],
  transform: {
    "^.+\\.ts$": "ts-jest",
  },
  moduleNameMapper: {
    "^ui/(.*)$": "<rootDir>/ui/$1",
  },
  testEnvironment: "jsdom",
  testRegex: "(/__tests__/.*|(\\.|/)(test))\\.ts$",
  moduleFileExtensions: ["ts", "js", "json"],
};
