import globalClassNames, { ClassNames as GlobalClassNames } from "....style.d";
declare const classNames: typeof globalClassNames & {
  readonly pageLoading: "pageLoading";
  readonly pageLoadingEnter: "pageLoadingEnter";
  readonly pageLoadingEnterActive: "pageLoadingEnterActive";
  readonly pageLoadingExit: "pageLoadingExit";
  readonly pageLoadingExitActive: "pageLoadingExitActive";
};
export default classNames;
export type ClassNames =
  | "pageLoading"
  | "pageLoadingEnter"
  | "pageLoadingEnterActive"
  | "pageLoadingExit"
  | "pageLoadingExitActive"
  | GlobalClassNames;
