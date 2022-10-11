import type { RootState } from "../types";
import type { DestinyAccount } from "./types";

export const accountsSelector = (state: RootState): DestinyAccount[] => state.accounts.accounts;