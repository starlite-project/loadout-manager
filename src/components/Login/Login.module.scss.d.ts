import globalClassNames, { ClassNames as GlobalClassNames } from "....style.d";
declare const classNames: typeof globalClassNames & {
  readonly billboard: "billboard";
  readonly auth: "auth";
};
export default classNames;
export type ClassNames = "billboard" | "auth" | GlobalClassNames;
