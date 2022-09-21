import globalClassNames, { ClassNames as GlobalClassNames } from "./style.d";
declare const classNames: typeof globalClassNames & {
  readonly wrapper: "wrapper";
};
export default classNames;
export type ClassNames = "wrapper" | GlobalClassNames;
