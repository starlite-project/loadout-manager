import globalClassNames, { ClassNames as GlobalClassNames } from "....style.d";
declare const classNames: typeof globalClassNames & {
  readonly billboard: "billboard";
};
export default classNames;
export type ClassNames = "billboard" | GlobalClassNames;
