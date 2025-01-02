import { JestConfigWithTsJest } from "ts-jest";

const config: JestConfigWithTsJest = {
  preset: "ts-jest",
  roots: ["<rootDir>/packages/moss-tabs"],
  modulePaths: ["<rootDir>/packages/moss-tabs/src"],
  displayName: { name: "moss-tabs", color: "blue" },
  rootDir: "../../",
  collectCoverageFrom: ["<rootDir>/packages/moss-tabs/src/**/*.{js,jsx,ts,tsx}"],
  setupFiles: ["<rootDir>/packages/moss-tabs/src/__tests__/__mocks__/resizeObserver.js"],
  setupFilesAfterEnv: ["<rootDir>/jest-setup.ts"],
  coveragePathIgnorePatterns: ["/node_modules/"],
  modulePathIgnorePatterns: [
    "<rootDir>/packages/moss-tabs/src/__tests__/__mocks__",
    "<rootDir>/packages/moss-tabs/src/__tests__/__test_utils__",
  ],
  coverageDirectory: "<rootDir>/packages/moss-tabs/coverage/",
  testResultsProcessor: "jest-sonar-reporter",
  testEnvironment: "jsdom",
  transform: {
    "^.+\\.tsx?$": [
      "ts-jest",
      {
        tsconfig: "<rootDir>/tsconfig.test.json",
      },
    ],
  },
};

export default config;
