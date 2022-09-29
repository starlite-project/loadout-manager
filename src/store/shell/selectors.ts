import type { RootState } from "../types";

export const routerLocationSelector = (state: RootState): string | undefined => state.shell.routerLocation;